//use std::fmt;
//use std::ptr;
use std::str::Chars;
//use std::error::Error;
//use errors::ExtractResult;
//use errors::ExtractError;
use token::WhitespaceKind;
use token::XmlToken;


type XmlTokenVec = Vec<XmlToken>;


pub trait XmlTokenize {
	fn tokenize(&self) -> XmlTokenVec;
}

trait XmlParse {
	fn parse_value(value: &char, iter: &mut Chars, tokens: &mut XmlTokenVec) -> Option<char>;
	fn parse_name(value: &char, iter: &mut Chars, tokens: &mut XmlTokenVec) -> Option<char>;
	fn parse_whitespace(value: &char, iter: &mut Chars, tokens: &mut XmlTokenVec) -> Option<char>;
}

trait XmlValidation {
	fn is_valid_in_xml(&self) -> bool;
	fn is_digit_in_xml(&self) -> bool;
	fn is_valid_first_char_in_element_name(&self) -> bool;
	fn is_valid_char_in_element_name(&self) -> bool;
	fn is_begin(&self) -> bool;
	fn is_end(&self) -> bool;
	fn is_close(&self) -> bool;
	fn is_quote(&self) -> bool;
}

impl XmlValidation for char {
	fn is_valid_in_xml(&self) -> bool {
		match self {
			'\u{0001}' ... '\u{D7FF}' |
			'\u{E000}' ... '\u{FFFD}' => true,
			_ => false,
		}
	}
	fn is_digit_in_xml(&self) -> bool {
		match self {
			'\u{0030}' ... '\u{0039}' | 
			'\u{0660}' ... '\u{0669}' | 
			'\u{06F0}' ... '\u{06F9}' | 
			'\u{0966}' ... '\u{096F}' | 
			'\u{09E6}' ... '\u{09EF}' | 
			'\u{0A66}' ... '\u{0A6F}' | 
			'\u{0AE6}' ... '\u{0AEF}' | 
			'\u{0B66}' ... '\u{0B6F}' | 
			'\u{0BE7}' ... '\u{0BEF}' | 
			'\u{0C66}' ... '\u{0C6F}' | 
			'\u{0CE6}' ... '\u{0CEF}' | 
			'\u{0D66}' ... '\u{0D6F}' | 
			'\u{0E50}' ... '\u{0E59}' | 
			'\u{0ED0}' ... '\u{0ED9}' | 
			'\u{0F20}' ... '\u{0F29}' => true,
			_ => false,
		}
	}
	fn is_valid_first_char_in_element_name(&self) -> bool {
		match self {
			'a' ... 'z' |
			'A' ... 'Z' |
			'_' => true,
			_ => false,
		}
	}
	fn is_valid_char_in_element_name(&self) -> bool {
		match self {
			'a' ... 'z' |
			'A' ... 'Z' |
			'0' ... '9' |
			':' |
			'_' => true,
			_ => false,
		}
	}
	fn is_begin(&self) -> bool {
		*self == '<'
	}
	fn is_end(&self) -> bool {
		*self == '>'
	}
	fn is_close(&self) -> bool {
		*self == '/'
	}
	fn is_quote(&self) -> bool {
		*self == '\"'
	}
}

impl XmlParse for String {
	fn parse_value(value: &char, iter: &mut Chars, tokens: &mut XmlTokenVec) -> Option<char> {
		let mut next: Option<char>;
		let mut value: String = String::new();
		loop {
			next = iter.next();
			if next.is_some() {
				if next.unwrap().is_quote() {
					break;
				}
				else {
					value.push(next.unwrap());
				}
			}
			else {
				break;
			}
		}
		tokens.push(XmlToken::new_value(value));
		next.and_then(|c| 
			if c.is_quote() {
				tokens.push(XmlToken::new_quote());
				iter.next()
			}
			else {
				panic!("missing quote for attribute value");
			}
		)
	}
	fn parse_name(value: &char, iter: &mut Chars, tokens: &mut XmlTokenVec) -> Option<char> {
		use tokenizer::XmlValidation;

		let mut next: Option<char>;
		if value.is_valid_first_char_in_element_name() {
			let mut name: String = String::new();
			name.push(*value);
			loop {
				next = iter.next();
				if next.is_some() {
					let c = next.unwrap();
					if c.is_valid_char_in_element_name() {
						name.push(c);
					}
					else {
						break;
					}
				} 
				else {
					break;
				}
			} 
			tokens.push(XmlToken::new_name(name));
		}
		else {
			panic!("Invalid namespace character '{}'", value);
		}
		next
	}
	fn parse_whitespace(value: &char, iter: &mut Chars, tokens: &mut XmlTokenVec) -> Option<char> {
		// find all same as value
		assert!(value.is_whitespace());
		let mut next = iter.next();
		let mut count = 1;
		loop {
			if next.is_some() {
				let c = next.unwrap();
				if &c == value {
					count += 1;
					next = iter.next();
				}
				else {
					break;
				}
			}
			else {
				break;
			}
		}
		tokens.push(XmlToken::Whitespace(WhitespaceKind::from_char(*value, count).unwrap()));
		next
	}
}

impl XmlTokenize for String {
	fn tokenize(&self) -> XmlTokenVec {
		use token::XmlToken::*;
		let mut result = XmlTokenVec::new();
		let mut iter = self.chars();
		let mut next = iter.next();
		loop {
			next = next.as_ref().and_then(|c| 
				if c.is_whitespace() {
					Self::parse_whitespace(c, &mut iter, &mut result)
				}
				else {
					let token = XmlToken::from_char(*c);
					match token {
						Some(Begin) |
						Some(Close) |
						Some(End) |
						Some(Assign) => { 
							result.push(token.unwrap());
							iter.next()
						},
						Some(Quote) => {
							result.push(token.unwrap());
							Self::parse_value(c, &mut iter, &mut result)
						},
						_ => {
							Self::parse_name(c, &mut iter, &mut result)
						},
					}
				}
			);
			if next.is_none() {
				break;
			}
		}
		result
	}
}


pub fn new_whitespace_kind(value: &char, count: usize) -> Option<WhitespaceKind> {
	match value {
		'\u{0020}' => Some(WhitespaceKind::Space(count)),
		'\u{0009}' => Some(WhitespaceKind::Tab(count)),
		'\u{000A}' => Some(WhitespaceKind::LF(count)),
		'\u{000D}' => Some(WhitespaceKind::CR(count)),
		_ => None
	}
}


#[cfg(test)]
mod tests {
	use token::WhitespaceKind;
	use token::XmlToken;
	use super::XmlTokenize;


	#[test]
	fn space_token_should_be_updated() {
		let whitespace = WhitespaceKind::Space(10);
		assert_eq!(whitespace, WhitespaceKind::Space(10));
	}
	#[test]
	fn new_tokenizer() {
		let tokenizer = "".to_string().tokenize();
		assert_eq!(tokenizer.len(), 0);
	}
	#[test]
	fn new_tokenizer_one_space() {
		let tokenizer = " ".to_string().tokenize();
		{
			let mut iter = tokenizer.iter();
			assert_eq!(iter.next().unwrap(), &XmlToken::Whitespace(WhitespaceKind::Space(1)));
		}
	}
	#[test]
	fn new_tokenizer_xml_element() {
		let tokenizer = "<element>".to_string().tokenize();
		{
			let mut iter = tokenizer.iter();
			assert_eq!(iter.next().unwrap(), &XmlToken::new_begin());
			assert_eq!(iter.next().unwrap(), &XmlToken::new_name("element".to_string()));
			assert_eq!(iter.next().unwrap(), &XmlToken::new_end());
		}
	}
	#[test]
	fn new_tokenizer_xml_element_to_string() {
		let tokenizer = "<element>".to_string().tokenize();
		let mut text = String::new();
		for token in tokenizer {
			text = text + &token.to_string();
		}
		assert_eq!(text, "<element>".to_string());
	}
	#[test]
	fn new_tokenizer_xml_full_element() {
		let tokenizer = "<element/>".to_string().tokenize();
		{
			let mut iter = tokenizer.iter();
			assert_eq!(iter.next().unwrap(), &XmlToken::new_begin());
			assert_eq!(iter.next().unwrap(), &XmlToken::new_name("element".to_string()));
			assert_eq!(iter.next().unwrap(), &XmlToken::new_close());
			assert_eq!(iter.next().unwrap(), &XmlToken::new_end());
		}
	}
	#[test]
	fn new_tokenizer_xml_element_in_element() {
		let tokenizer = "<element><level/></element>".to_string().tokenize();
		{
			let mut iter = tokenizer.iter();
			assert_eq!(iter.next().unwrap(), &XmlToken::new_begin());
			assert_eq!(iter.next().unwrap(), &XmlToken::new_name("element".to_string()));
			assert_eq!(iter.next().unwrap(), &XmlToken::new_end());
			assert_eq!(iter.next().unwrap(), &XmlToken::new_begin());
			assert_eq!(iter.next().unwrap(), &XmlToken::new_name("level".to_string()));
			assert_eq!(iter.next().unwrap(), &XmlToken::new_close());
			assert_eq!(iter.next().unwrap(), &XmlToken::new_end());
			assert_eq!(iter.next().unwrap(), &XmlToken::new_begin());
			assert_eq!(iter.next().unwrap(), &XmlToken::new_close());
			assert_eq!(iter.next().unwrap(), &XmlToken::new_name("element".to_string()));
			assert_eq!(iter.next().unwrap(), &XmlToken::new_end());
		}
	}
	#[test]
	fn new_tokenizer_xml_full_element_with_attribute() {
		let tokenizer = "<element attribute=\"value\"/>".to_string().tokenize();
		assert_eq!(tokenizer.len(), 10);
		{
			let mut iter = tokenizer.iter();
			assert_eq!(iter.next().unwrap(), &XmlToken::new_begin());
			assert_eq!(iter.next().unwrap(), &XmlToken::new_name("element".to_string()));
			assert_eq!(iter.next().unwrap(), &XmlToken::Whitespace(WhitespaceKind::Space(1)));
			assert_eq!(iter.next().unwrap(), &XmlToken::new_name("attribute".to_string()));
			assert_eq!(iter.next().unwrap(), &XmlToken::new_assign());
			assert_eq!(iter.next().unwrap(), &XmlToken::new_quote());
			assert_eq!(iter.next().unwrap(), &XmlToken::new_value("value".to_string()));
			assert_eq!(iter.next().unwrap(), &XmlToken::new_quote());
			assert_eq!(iter.next().unwrap(), &XmlToken::new_close());
			assert_eq!(iter.next().unwrap(), &XmlToken::new_end());
		}
	}
	#[test]
	fn new_tokenizer_xml_full_element_with_attribute_to_string() {
		let tokenizer = "<element attribute=\"value\"/>".to_string().tokenize();
		assert_eq!(tokenizer.len(), 10);
		let mut text = String::new();
		for token in tokenizer {
			text += &token.to_string();
		}
		assert_eq!(text, "<element attribute=\"value\"/>".to_string());
	}
}
