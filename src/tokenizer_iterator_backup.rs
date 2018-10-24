//use std::fmt;
//use std::ptr;
use std::str::Chars;
//use std::error::Error;
//use errors::ExtractResult;
//use errors::ExtractError;
//use token::WhitespaceKind;
//use token::SymbolKind;
//use token::XmlToken;


#[derive(Debug, Copy, Clone, PartialEq)] 
pub enum WhitespaceKind {
	Space(usize),
	Tab(usize),
	CR(usize),
	LF(usize),
	FF(usize),
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

trait Extractor {
	fn peek_first(&mut self) -> Option<char>;
//	fn extract_whitespace(&mut self) -> Option<WhitespaceKind>;
}


impl <'a> Extractor for Chars<'a> {
	fn peek_first(&mut self) -> Option<char> {
		let mut pk = self.peekable();
		pk.peek().and_then(|c| Some(*c))
	}
//	fn extract_whitespace(&mut self) -> Option<WhitespaceKind> {
//		let mut pk = self.peekable();
//		let mut count: usize = 0;
//		let ws = pk.peek().unwrap_or(&'\u{0000}').clone();
//		if ws.is_whitespace() {
//			while pk.peek() == Some(&ws) { pk.next(); count += 1 };
//			new_whitespace_kind(&ws, count)
//		}
//		else {
//			None
//		}
//	}
}


#[cfg(test)]
mod tests {
	use std::str::Chars;
	use super::Extractor;
	use token::WhitespaceKind;

	#[test]
	fn peek_first_test() {
		let mut iterator: Chars;
		iterator = "".chars();
		assert_eq!(iterator.peek_first(), None);
		assert_eq!(iterator.as_str(), "".to_string());

		iterator = "A".chars();
		assert_eq!(iterator.peek_first(), Some('A'));
		assert_eq!(iterator.as_str(), "A".to_string());

		iterator = "AB".chars();
		assert_eq!(iterator.peek_first(), Some('A'));
		assert_eq!(iterator.as_str(), "AB".to_string());
	}


/*
	#[test]
	fn extract_ws_token() {
		let mut iterator: Chars;
		iterator = "".chars();
		assert_eq!(iterator.extract_whitespace(), None);
		assert_eq!(iterator.as_str(), "".to_string());

		iterator = "   ".chars();
		assert_eq!(iterator.extract_whitespace(), Some(WhitespaceKind::Space(3)));
		assert_eq!(iterator.as_str(), "".to_string());

		iterator = "\u{0020}\u{0020}\u{0020}".chars();
		assert_eq!(iterator.extract_whitespace(), Some(WhitespaceKind::Space(3)));
		assert_eq!(iterator.as_str(), "".to_string());

		iterator = "\u{0009}\u{0009}\u{0009}\u{0009}".chars();
		assert_eq!(iterator.extract_whitespace(), Some(WhitespaceKind::Tab(4)));
		assert_eq!(iterator.as_str(), "".to_string());

		iterator = "\u{000A}\u{000A}\u{0009}\u{0009}".chars();
		assert_eq!(iterator.extract_whitespace(), Some(WhitespaceKind::LF(2)));
		assert_eq!(iterator.as_str(), "\u{0009}\u{0009}".to_string());
//		iterator = "\u{000D}\u{000A}\u{0009}\u{0009}".to_string();
//		assert_eq!(iterator.extract_white_space(), Some(WhitespaceKind::CR(1)));
//		assert_eq!(iterator, "\u{000A}\u{0009}\u{0009}".to_string());
//		iterator = "\u{000C}\u{000C}\u{0000}\u{0000}".to_string();
//		assert_eq!(iterator.extract_white_space(), Some(WhitespaceKind::FF(2)));
//		assert_eq!(iterator, "\u{0000}\u{0000}".to_string());
	} 
*/

} 

