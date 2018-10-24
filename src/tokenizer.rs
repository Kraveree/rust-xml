//use std::fmt;
//use std::ptr;
//use std::str::Chars;
//use std::error::Error;
//use errors::ExtractResult;
//use errors::ExtractError;
use token::WhitespaceKind;
use token::SymbolKind;
use token::XmlToken;


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


trait Extractor {
	fn peek_first(&self) -> Option<char>;
	fn count_same(&self, value: &char) -> usize;
	fn take_until(&mut self, contains: &String) -> String;
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

	fn take_until(&mut self, contains: &String) -> String {
		self.splitn(1, |c| contains.contains(c)).collect().to_string()
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
	fn extract_ident(&self, start: &char, end: &char) -> Option<String>;
}


impl IdentExtract for String {
	fn check_ident(&self, start: &char, end: &char) -> Option<String> {
		None
	}
	fn extract_ident(&self, start: &char, end: &char) -> Option<String>{
		None
	}
}


trait ElementExtract: Extractor {
	fn check_element_token(&self) -> Option<XmlToken>;
	fn extract_element_token(&self) -> Option<XmlToken>;
}


impl ElementExtract for String {
	fn check_element_token(&self) -> Option<XmlToken> {
		None
	}

	fn extract_element_token(&self) -> Option<XmlToken> {
		None
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


#[cfg(test)]
mod tests {
	use token::WhitespaceKind; 
	use token::SymbolKind; 
	use token::XmlToken; 
	use super::WhitespaceExtract;
	use super::XmlTokenExtract;
	use super::SymbolExtract;
	use super::IdentExtract;

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
	fn extract_ident() {
		let ident: String = "\"ident\"".to_string();
		assert_eq!(ident.check_ident(&'\"', &'\"'), Some("ident".to_string()));
	} 
}
