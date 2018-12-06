//use std::fmt;
//use std::ptr;
//use std::str::Chars;
//use std::error::Error;
//use errors::ExtractResult;
//use errors::ExtractError;
use token::WhitespaceKind;
use token::SymbolKind;
use token::XmlToken;


type XmlTokenVec<'a> = Vec<XmlToken<'a>>;


pub trait XmlTokenize {
	fn tokenize(&mut self) -> XmlTokenVec;
}


pub fn new_whitespace_kind(value: &char, count: usize) -> Option<WhitespaceKind> {
	match value {
		'\u{0020}' => Some(WhitespaceKind::Space(count)),
		'\u{0009}' => Some(WhitespaceKind::Tab(count)),
		'\u{000A}' => Some(WhitespaceKind::LF(count)),
		'\u{000D}' => Some(WhitespaceKind::CR(count)),
		'\u{000C}' => Some(WhitespaceKind::FF(count)),
		_ => None
	}
}


trait Equals: Sized {
	fn equals(&self, value: Self) -> Option<Self>;
}


impl Equals for char {
	fn equals(&self, value: char) -> Option<char> {
		if *self == value {
			Some(*self)
		}
		else {
			None
		}
	}
}

impl<'a> Equals for &'a char {
	fn equals(&self, value: &char) -> Option<&'a char> {
		if **self == *value {
			Some(self)
		}
		else {
			None
		}
	}
}

trait Extractor {
	fn peek_first(&self) -> Option<char>;
	fn count_same(&self, value: &char) -> usize;
	fn take_while_contains(&mut self, contains: &String) -> Option<String>;
	fn reverse(&mut self) -> &String;
	fn take_until<F>(&mut self, f: F) -> Option<String>
		where F: Fn(char) -> bool;
}


impl Extractor for String {
	fn peek_first(&self) -> Option<char> {
		self.chars().peekable().peek().cloned()
	}

	fn count_same(&self, value: &char) -> usize {
		let mut count: usize = 0;
		let mut peekable = self.chars().peekable();

		while peekable.peek() == Some(value) {
			count += 1;
			peekable.next();
		};
		count
	}


	fn take_until<F>(&mut self, f: F) -> Option<String>
		where F: Fn(char) -> bool {
		let mut result: String = "".to_string();
		{
			let iter = self.chars();
			for c in iter {
				if !f(c) {
					result.push(c);
				}
				else {
					break;
				}
			}
		}
		if result.len() > 0 {
			self.drain(..result.len());
			Some(result)
		}
		else {
			None
		}
	}


	fn take_while_contains(&mut self, contains: &String) -> Option<String> {
		self.take_until(|c| !contains.contains(c))
	}


	fn reverse(&mut self) -> &String {
		let rev = String::from(self.chars().rev().collect::<String>());
		*self = rev;
		self
	}
}


trait WhitespaceExtract : Extractor {
	fn peek_whitespace(&self) -> Option<char>;
	fn check_white_space(&self) -> Option<WhitespaceKind>;
	fn extract_white_space(&mut self) -> Option<WhitespaceKind>;
}


impl WhitespaceExtract for String {
	fn peek_whitespace(&self) -> Option<char> {
		self.peek_first()
			.and_then(|c| if c.is_whitespace() {
						Some(c)
				      }
				      else {
						None
				      })
	}

	fn check_white_space(&self) -> Option<WhitespaceKind> {
		self.peek_whitespace().and_then(|c| {
			let count = self.count_same(&c);
			new_whitespace_kind(&c, count)
		})
	}

	fn extract_white_space(&mut self) -> Option<WhitespaceKind> {
		self.peek_whitespace().and_then(|c| {
			let count = self.count_same(&c);
			self.drain(..count);
			new_whitespace_kind(&c, count)
		})
	}
}


trait SymbolExtract : Extractor {
	fn peek_symbol(&self) -> Option<char>;
	fn is_symbol(value: &char) -> bool;
	fn check_symbol(&self) -> Option<SymbolKind>;
	fn extract_symbol(&mut self) -> Option<SymbolKind>;
}

static SYMBOLS: &'static str = "!?@#$%^&*()-+=|\\/<>,.~[]{}";

impl SymbolExtract for String {
	fn is_symbol(value: &char) -> bool {
		SYMBOLS.contains(*value)
	}

	fn peek_symbol(&self) -> Option<char> {
		self.peek_first()
			.and_then(|c| if Self::is_symbol(&c) {
						Some(c)
				      }
				      else {
						None
				      })
	}

	fn check_symbol(&self) -> Option<SymbolKind> {
		self.peek_first().and_then(|c| Some(SymbolKind::Symbol(c)))
	}

	fn extract_symbol(&mut self) -> Option<SymbolKind> {
		let result = self.check_symbol();
		if result.is_some() { self.drain(..1); };
		result
	}
}


trait IdentExtract: Extractor + SymbolExtract {
	fn check_ident(&self, start: &char, end: &char) -> Option<String>;
	fn extract_ident(&mut self, start: &char, end: &char) -> Option<String>;
}


impl IdentExtract for String {
	fn check_ident(&self, start: &char, end: &char) -> Option<String> {
		if self.starts_with(*start) {
			let mut iter = self.chars();
			iter.next();
			iter.collect::<String>().take_until(|c| c == *end)
		}
		else {
			None
		}
	}

	fn extract_ident(&mut self, start: &char, end: &char) -> Option<String> {
		let result = self.check_ident(start, end);
		if result.is_some() {
			self.drain(..result.clone().unwrap().len()+2);
		}
		result
	}
}


trait ElementExtract: Extractor {
	fn check_element_token(&self) -> Option<XmlToken>;
	fn extract_element_token(&mut self) -> Option<XmlToken>;
}


impl ElementExtract for String {
	fn check_element_token(&self) -> Option<XmlToken> {
		let mut result = None;
		if self.starts_with('<') {
			let mut iter = self.chars();
			iter.next();
			let ident = iter.collect::<String>().take_until(|c| c.is_whitespace() || c == '/' || c== '>');
			if ident.is_some() {
				result = Some(XmlToken::StartElement(ident.unwrap()));
			}
		}
		result
	}

	fn extract_element_token(&mut self) -> Option<XmlToken> {
		let mut count: usize = 0;
		let mut result = None;
		if self.starts_with('<') {
			let mut iter = self.chars();
			iter.next();
			let ident = iter.collect::<String>().take_until(|c| c.is_whitespace() || c == '/' || c== '>').unwrap_or("".to_string());
			count = ident.len()+1;
			if count > 0 {
				result = Some(XmlToken::StartElement(ident));
			}
		}
		self.drain(..count);
		result
	}
}


trait XmlTokenExtract: WhitespaceExtract + ElementExtract + SymbolExtract {
	fn check_xml_token(&self) -> Option<XmlToken>;
	fn extract_xml_token(&mut self) -> Option<XmlToken>;
}


impl XmlTokenExtract for String {
	fn check_xml_token(&self) -> Option<XmlToken>{
		let whitespace = self.check_white_space();
		if whitespace.is_some() {
			Some(XmlToken::Whitespace(whitespace.unwrap()))
		}
		else {
			let element = self.check_element_token();
			if element.is_some() {
				element
			}
			else {
				None
			}
		}
	}

	fn extract_xml_token(&mut self) -> Option<XmlToken>{
		let whitespace = self.extract_white_space();
		if whitespace.is_some() {
			Some(XmlToken::Whitespace(whitespace.unwrap()))
		}
		else {
			let element = self.check_element_token();
			if element.is_some() {
				element
			}
			else {
				None
			}
		}
	}
}


impl XmlTokenize for String {
	fn tokenize(&mut self) -> XmlTokenVec {
		let clone = self.clone();
		let mut token: Option<XmlToken>;
		let mut result = Vec::new();
		{
			loop {
				token = clone.extract_xml_token();
				{
					if token.is_some() {
						result.push(token.unwrap());
					}
					else {
						break;
					}
				}
			}
		}
		result
	}
}



#[cfg(test)]
mod tests {
	use token::WhitespaceKind;
	use token::SymbolKind;
	use token::XmlToken;
	use super::WhitespaceExtract;
	use super::XmlTokenExtract;
	use super::SymbolExtract;
	use super::IdentExtract;
	use super::ElementExtract;
	use super::Extractor;
	use super::XmlTokenize;

	#[test]
	fn extractor_peek_first() {
		let mut s: String;
		s = "".to_string();
		assert_eq!(s.peek_first(), None);
		assert_eq!(s, "");
		s = "aBc".to_string();
		assert_eq!(s.peek_first(), Some('a'));
		assert_eq!(s, "aBc");
	}

	#[test]
	fn extractor_count_same() {
		let mut s: String;

		s = "".to_string();
		assert_eq!(s.count_same(&'a'), 0);
		assert_eq!(s, "");

		s = "aaa".to_string();
		assert_eq!(s.count_same(&'a'), 3);
		assert_eq!(s, "aaa");

		s = "aaaabbccdd".to_string();
		assert_eq!(s.count_same(&'a'), 4);
		assert_eq!(s, "aaaabbccdd");
	}

	#[test]
	fn extractor_take_while_contains() {
		let mut s: String;
		let mut contains: String;
		s = "".to_string();
		contains = "".to_string();
		assert_eq!(s.take_while_contains(&contains), None);
		assert_eq!(s, "");
		assert_eq!(contains, "");

		s = "abcd!!".to_string();
		contains = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_".to_string();
		assert_eq!(s.take_while_contains(&contains), Some("abcd".to_string()));
		assert_eq!(s, "!!");
		assert_eq!(contains, "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_");
	}

	#[test]
	fn extractor_take_until() {
		let mut s: String;
		s = "".to_string();
		assert_eq!(s.take_until(|c| c==c), None);
		assert_eq!(s, "");

		s = "dcba!!".to_string();
		assert_eq!(s.take_until(|c| c == '!'), Some("dcba".to_string()));
		assert_eq!(s, "!!");

		s = "!abcd!!".to_string();
		assert_eq!(s.take_until(|c| c == '!'), None);
		assert_eq!(s, "!abcd!!");

		s = "-ab cd-!!!".to_string();
		assert_eq!(s.take_until(|c| c == '!'), Some("-ab cd-".to_string()));
		assert_eq!(s, "!!!");
	}


	#[test]
	fn extractor_reverse() {
		let mut s: String;
		s = "".to_string();
		assert_eq!(s.reverse(), "");
		assert_eq!(s, "");

		s = "abcdefg".to_string();
		assert_eq!(s.reverse(), "gfedcba");
		assert_eq!(s, "gfedcba");

		s = "來ぬ人の秋のけしきやふけぬらん恨みによはる松蟲の聲".to_string();
		assert_eq!(s.reverse(), "聲の蟲松るはよにみ恨んらぬけふやきしけの秋の人ぬ來");
		assert_eq!(s, "聲の蟲松るはよにみ恨んらぬけふやきしけの秋の人ぬ來");
	}

	#[test]
	fn space_token() {
		let whitespace = WhitespaceKind::Space(10);
		assert_eq!(whitespace, WhitespaceKind::Space(10));
	}

	#[test]
	fn extract_ws_token() {
		let mut ws: String;

		ws = "".to_string();
		assert_eq!(ws.extract_white_space(), None);
		assert_eq!(ws, "".to_string());

		ws = "   ".to_string();
		assert_eq!(ws.extract_white_space(), Some(WhitespaceKind::Space(3)));
		assert_eq!(ws, "".to_string());

		ws = "\u{0020}\u{0020}\u{0020}".to_string();
		assert_eq!(ws.extract_white_space(), Some(WhitespaceKind::Space(3)));
		assert_eq!(ws, "".to_string());

		ws = "\u{0009}\u{0009}\u{0009}\u{0009}".to_string();
		assert_eq!(ws.extract_white_space(), Some(WhitespaceKind::Tab(4)));
		assert_eq!(ws, "".to_string());

		ws = "\u{000A}\u{000A}\u{0009}\u{0009}".to_string();
		assert_eq!(ws.extract_white_space(), Some(WhitespaceKind::LF(2)));
		assert_eq!(ws, "\u{0009}\u{0009}".to_string());

		ws = "\u{000D}\u{000A}\u{0009}\u{0009}".to_string();
		assert_eq!(ws.extract_white_space(), Some(WhitespaceKind::CR(1)));
		assert_eq!(ws, "\u{000A}\u{0009}\u{0009}".to_string());

		ws = "\u{000C}\u{000C}\u{0000}\u{0000}".to_string();
		assert_eq!(ws.extract_white_space(), Some(WhitespaceKind::FF(2)));
		assert_eq!(ws, "\u{0000}\u{0000}".to_string());
	}

	#[test]
	fn extract_xml_token() {
		let mut ws: String;

		ws = "".to_string();
		assert_eq!(ws.extract_xml_token(), None);
		assert_eq!(ws, "".to_string());

		ws = "   ".to_string();
		assert_eq!(ws.extract_xml_token(), Some(XmlToken::Whitespace(WhitespaceKind::Space(3))));
		assert_eq!(ws, "");

		ws = "\u{0020}\u{0020}\u{0020}".to_string();
		assert_eq!(ws.extract_xml_token(), Some(XmlToken::Whitespace(WhitespaceKind::Space(3))));
		assert_eq!(ws, "");

		ws = "\u{0009}\u{0009}\u{0009}\u{0009}".to_string();
		assert_eq!(ws.extract_xml_token(), Some(XmlToken::Whitespace(WhitespaceKind::Tab(4))));
		assert_eq!(ws, "");

		ws = "\u{000A}\u{000A}\u{0009}\u{0009}".to_string();
		assert_eq!(ws.extract_xml_token(), Some(XmlToken::Whitespace(WhitespaceKind::LF(2))));
		assert_eq!(ws, "\u{0009}\u{0009}");

		ws = "\u{000D}\u{000A}\u{0009}\u{0009}".to_string();
		assert_eq!(ws.extract_xml_token(), Some(XmlToken::Whitespace(WhitespaceKind::CR(1))));
		assert_eq!(ws, "\u{000A}\u{0009}\u{0009}");

		ws = "\u{000C}\u{000C}\u{0000}\u{0000}".to_string();
		assert_eq!(ws.extract_xml_token(), Some(XmlToken::Whitespace(WhitespaceKind::FF(2))));
		assert_eq!(ws, "\u{0000}\u{0000}");
	}

	#[test]
	fn check_symbol_token() {
		let mut symbol: String;

		symbol = "".to_string();
		assert_eq!(symbol.peek_symbol(), None);

		symbol = "!".to_string();
		assert_eq!(symbol.peek_symbol(), Some('!'));

		symbol = "?".to_string();
		assert_eq!(symbol.check_symbol(), Some(SymbolKind::Symbol('?')));
		assert_eq!(symbol, "?");
	}

	#[test]
	fn check_ws_token() {
		let mut ws: String;

		ws = "".to_string();
		assert_eq!(ws.check_white_space(), None);
		assert_eq!(ws, "".to_string());

		ws = "   ".to_string();
		assert_eq!(ws.check_white_space(), Some(WhitespaceKind::Space(3)));
		assert_eq!(ws, "   ".to_string());

		ws = "\u{0020}\u{0020}\u{0020}".to_string();
		assert_eq!(ws.check_white_space(), Some(WhitespaceKind::Space(3)));
		assert_eq!(ws, "\u{0020}\u{0020}\u{0020}");

		ws = "\u{0009}\u{0009}\u{0009}\u{0009}".to_string();
		assert_eq!(ws.check_white_space(), Some(WhitespaceKind::Tab(4)));
		assert_eq!(ws, "\u{0009}\u{0009}\u{0009}\u{0009}");

		ws = "\u{000A}\u{000A}\u{0009}\u{0009}".to_string();
		assert_eq!(ws.check_white_space(), Some(WhitespaceKind::LF(2)));
		assert_eq!(ws, "\u{000A}\u{000A}\u{0009}\u{0009}".to_string());

		ws = "\u{000D}\u{000A}\u{0009}\u{0009}".to_string();
		assert_eq!(ws.check_white_space(), Some(WhitespaceKind::CR(1)));
		assert_eq!(ws, "\u{000D}\u{000A}\u{0009}\u{0009}");

		ws = "\u{000C}\u{000C}\u{0000}\u{0000}".to_string();
		assert_eq!(ws.check_white_space(), Some(WhitespaceKind::FF(2)));
		assert_eq!(ws, "\u{000C}\u{000C}\u{0000}\u{0000}");
	}

	#[test]
	fn check_xml_token() {
		let mut xmltoken: String;

		xmltoken = "".to_string();
		assert_eq!(xmltoken.check_xml_token(), None);
		assert_eq!(xmltoken, "".to_string());

		xmltoken = "   ".to_string();
		assert_eq!(xmltoken.check_xml_token(), Some(XmlToken::Whitespace(WhitespaceKind::Space(3))));
		assert_eq!(xmltoken, "   ".to_string());

		xmltoken = "\u{0020}\u{0020}\u{0020}".to_string();
		assert_eq!(xmltoken.check_xml_token(), Some(XmlToken::Whitespace(WhitespaceKind::Space(3))));
		assert_eq!(xmltoken, "\u{0020}\u{0020}\u{0020}");

		xmltoken = "\u{0009}\u{0009}\u{0009}\u{0009}".to_string();
		assert_eq!(xmltoken.check_xml_token(), Some(XmlToken::Whitespace(WhitespaceKind::Tab(4))));
		assert_eq!(xmltoken, "\u{0009}\u{0009}\u{0009}\u{0009}");

		xmltoken = "\u{000A}\u{000A}\u{0009}\u{0009}".to_string();
		assert_eq!(xmltoken.check_xml_token(), Some(XmlToken::Whitespace(WhitespaceKind::LF(2))));
		assert_eq!(xmltoken, "\u{000A}\u{000A}\u{0009}\u{0009}");

		xmltoken = "\u{000D}\u{000A}\u{0009}\u{0009}".to_string();
		assert_eq!(xmltoken.check_xml_token(), Some(XmlToken::Whitespace(WhitespaceKind::CR(1))));
		assert_eq!(xmltoken, "\u{000D}\u{000A}\u{0009}\u{0009}");

		xmltoken = "\u{000C}\u{000C}\u{0000}\u{0000}".to_string();
		assert_eq!(xmltoken.check_xml_token(), Some(XmlToken::Whitespace(WhitespaceKind::FF(2))));
		assert_eq!(xmltoken, "\u{000C}\u{000C}\u{0000}\u{0000}");
	}

	#[test]
	fn extract_ws_token_in_sequence() {
		let mut ws: String = "\u{0020}\u{0009}\u{000A}\u{000D}\u{000C}".to_string();

		assert_eq!(ws.extract_white_space(), Some(WhitespaceKind::Space(1)));
		assert_eq!(ws, "\u{0009}\u{000A}\u{000D}\u{000C}".to_string());
		assert_eq!(ws.extract_white_space(), Some(WhitespaceKind::Tab(1)));
		assert_eq!(ws, "\u{000A}\u{000D}\u{000C}");
		assert_eq!(ws.extract_white_space(), Some(WhitespaceKind::LF(1)));
		assert_eq!(ws, "\u{000D}\u{000C}");
		assert_eq!(ws.extract_white_space(), Some(WhitespaceKind::CR(1)));
		assert_eq!(ws, "\u{000C}");
		assert_eq!(ws.extract_white_space(), Some(WhitespaceKind::FF(1)));
		assert_eq!(ws, "");
	}

	#[test]
	fn extract_xml_token_in_sequence() {
		let mut xmltoken: String = "\u{0020}\u{0009}\u{000A}\u{000D}\u{000C}".to_string();

		assert_eq!(xmltoken.extract_xml_token(), Some(XmlToken::Whitespace(WhitespaceKind::Space(1))));
		assert_eq!(xmltoken, "\u{0009}\u{000A}\u{000D}\u{000C}");
		assert_eq!(xmltoken.extract_xml_token(), Some(XmlToken::Whitespace(WhitespaceKind::Tab(1))));
		assert_eq!(xmltoken, "\u{000A}\u{000D}\u{000C}");
		assert_eq!(xmltoken.extract_xml_token(), Some(XmlToken::Whitespace(WhitespaceKind::LF(1))));
		assert_eq!(xmltoken, "\u{000D}\u{000C}");
		assert_eq!(xmltoken.extract_xml_token(), Some(XmlToken::Whitespace(WhitespaceKind::CR(1))));
		assert_eq!(xmltoken, "\u{000C}");
		assert_eq!(xmltoken.extract_xml_token(), Some(XmlToken::Whitespace(WhitespaceKind::FF(1))));
		assert_eq!(xmltoken, "");
	}

	#[test]
	fn check_ident() {
		let mut ident: String = "\"ident\"".to_string();

		assert_eq!(ident.check_ident(&'\"', &'\"'), Some("ident".to_string()));
		assert_eq!(ident, "\"ident\"");

		ident = "\"ident\" with trailing".to_string();
		assert_eq!(ident.check_ident(&'\"', &'\"'), Some("ident".to_string()));
		assert_eq!(ident, "\"ident\" with trailing");
	}

	#[test]
	fn extract_ident() {
		let mut ident: String = "\"ident\"".to_string();

		assert_eq!(ident.extract_ident(&'\"', &'\"'), Some("ident".to_string()));
		assert_eq!(ident, "");

		ident = "\"ident\" with trailing".to_string();
		assert_eq!(ident.extract_ident(&'\"', &'\"'), Some("ident".to_string()));
		assert_eq!(ident, " with trailing");
	}

	#[test]
	fn check_element() {
		let element: String = "<element/>".to_string();
		assert_eq!(element.check_element_token(), Some(XmlToken::StartElement("element".to_string())));
		assert_eq!(element, "<element/>");

		let element: String = "<element>".to_string();
		assert_eq!(element.check_element_token(), Some(XmlToken::StartElement("element".to_string())));
		assert_eq!(element, "<element>");

		let element: String = "<element attribute=\"value\" >".to_string();
		assert_eq!(element.check_element_token(), Some(XmlToken::StartElement("element".to_string())));
		assert_eq!(element, "<element attribute=\"value\" >");
	}


	#[test]
	fn extract_element() {
		let mut element: String = "<element/>".to_string();
		assert_eq!(element.extract_element_token(), Some(XmlToken::StartElement("element".to_string())));
		assert_eq!(element, "/>");

		element = "<element>".to_string();
		assert_eq!(element.extract_element_token(), Some(XmlToken::StartElement("element".to_string())));
		assert_eq!(element, ">");

		element = "<element attribute=\"value\" >".to_string();
		assert_eq!(element.extract_element_token(), Some(XmlToken::StartElement("element".to_string())));
		assert_eq!(element, " attribute=\"value\" >");
	}
	#[test]
	fn check_tokenize() {
		let mut xml: String = "    <element attribute=\"tekst\">Some text</element>  ".to_string();
		let tokens = xml.tokenize();
		assert_eq!(tokens.len(), 6);
		assert_eq!(tokens[0], XmlToken::Whitespace(WhitespaceKind::Space(4)));
		assert_eq!(tokens[1], XmlToken::StartElement("element".to_string()));
	}
}
