// use attribute::XmlAttribute;

#[derive(Debug, Copy, Clone, PartialEq, Eq)] 
pub enum WhitespaceKind {
	Space(usize),
	Tab(usize),
	CR(usize),
	LF(usize),
}


impl WhitespaceKind {
	pub fn to_char(self) -> char {
		use token::WhitespaceKind::*;
		match self {
			Space(_c) => ' ',
			Tab(_c) => '\t',
			CR(_c) => '\r',
			LF(_c) => '\n',
		}
	}
	pub fn to_string(self) -> String {
		use token::WhitespaceKind::*;
		match self {
			Space(c) => self.to_char().to_string().repeat(c),
			Tab(c) => self.to_char().to_string().repeat(c),
			CR(c) => self.to_char().to_string().repeat(c),
			LF(c) => self.to_char().to_string().repeat(c),
		}
	}
	pub fn from_char(value: char, count: usize) -> Option<Self> {
		use token::WhitespaceKind::*;
		match value {
			' ' => Some(Space(count)),
			'\t' => Some(Tab(count)),
			'\n' => Some(LF(count)),
			'\r' => Some(CR(count)),
			_ => None
		}
	}
	pub fn from_string(value: String) -> Option<Self> {
		if value.len() > 0 {
			WhitespaceKind::from_char(value.chars().next().unwrap(), value.len())
		}
		else {
			None
		}
	}
}



#[derive(Debug, Copy, Clone, PartialEq)] 
pub enum SymbolKind {
	Symbol(char),
}


#[derive(Debug, Clone, PartialEq, Eq)] 
pub enum XmlToken {
	Whitespace(WhitespaceKind), // any type of whitespace
	Begin, // <
	Name(String), // name
	Close, // /
	End, // >
	Value(String), // "value"
	Assign, // =
	Quote, // "
	Text(String), // any text
	Other(String), // future reference dtd etc
}

type OptionalXmlToken = Option<XmlToken>;


impl XmlToken {
	pub fn new_whitespace(value: String) -> OptionalXmlToken {
		let s = WhitespaceKind::from_string(value);
		s.and_then(|c| Some(XmlToken::Whitespace(c)))
	}
	pub fn new_name(name: String) -> XmlToken {
		XmlToken::Name(name)
	}
	pub fn new_begin() -> XmlToken {
		XmlToken::Begin
	}
	pub fn new_close() -> XmlToken {
		XmlToken::Close
	}
	pub fn new_end() -> XmlToken {
		XmlToken::End
	}
	pub fn new_value(value: String) -> XmlToken {
		XmlToken::Value(value)
	}
	pub fn new_assign() -> XmlToken {
		XmlToken::Assign
	}
	pub fn new_quote() -> XmlToken {
		XmlToken::Quote
	}
	pub fn new_text(value: String) -> XmlToken {
		XmlToken::Text(value)
	}
	pub fn to_string(self) -> String {
		use token::XmlToken::*;
		match self {
			Begin => format!("<"), 
			Name(s) => format!("{}", s),
			Close => format!("/"),
			End => format!(">"),
			Assign => format!("="),
			Value(v) => format!("{}", v),
			Quote => format!("\""),
			Text(s) => s,
			Other(s) => s,
			Whitespace(t) => t.to_string(),
		}
	}
	pub fn from_char(c: char) -> OptionalXmlToken {
		use token::XmlToken::*;
		match c {
			'<' => Some(Begin),
			'>' => Some(End),
			'=' => Some(Assign),
			'/' => Some(Close),
			'\"' => Some(Quote),
			_ => None,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn space_token() {
		let whitespace_space = WhitespaceKind::Space(10);
		assert_eq!(whitespace_space, WhitespaceKind::Space(10));
		assert_eq!(whitespace_space.to_char(), ' ');
		assert_eq!(whitespace_space.to_string(), " ".repeat(10));
		let whitespace_tab = WhitespaceKind::Tab(5);
		assert_eq!(whitespace_tab, WhitespaceKind::Tab(5));
		assert_eq!(whitespace_tab.to_char(), '\t');
		assert_eq!(whitespace_tab.to_string(), "\t".repeat(5));
		let whitespace_cr = WhitespaceKind::CR(1);
		assert_eq!(whitespace_cr, WhitespaceKind::CR(1));
		assert_eq!(whitespace_cr.to_char(), '\r');
		assert_eq!(whitespace_cr.to_string(), "\r".repeat(1));
		let whitespace_lf = WhitespaceKind::LF(15);
		assert_eq!(whitespace_lf, WhitespaceKind::LF(15));
		assert_eq!(whitespace_lf.to_char(), '\n');
		assert_eq!(whitespace_lf.to_string(), "\n".repeat(15));
	}
	#[test]
	fn xml_token_whitespace() {
		let token = XmlToken::new_whitespace("\t\t\t".to_string());
		assert_eq!(token, Some(XmlToken::Whitespace(WhitespaceKind::Tab(3))));
	}
	#[test]
	fn xml_token_whitespace_to_string() {
		let token = XmlToken::new_whitespace("\t\t\t".to_string()).unwrap();
		assert_eq!(token.to_string(), "\t\t\t".to_string());
	}
	#[test]
	fn xml_token_element() {
		let token = XmlToken::new_name("element".to_string());
		assert_eq!(token, XmlToken::Name("element".to_string()));
	}
	#[test]
	fn xml_token_element_to_string() {
		let token = XmlToken::new_name("element".to_string());
		assert_eq!(token.to_string(), "element".to_string());
	}
	#[test]
	fn xml_token_close() {
		let token = XmlToken::new_close();
		assert_eq!(token, XmlToken::Close);
	}
	#[test]
	fn xml_token_close_to_string() {
		let token = XmlToken::new_close();
		assert_eq!(token.to_string(), "/".to_string());
	}
	#[test]
	fn xml_token_end() {
		let token = XmlToken::new_end();
		assert_eq!(token, XmlToken::End);
	}
	#[test]
	fn xml_token_end_to_string() {
		let token = XmlToken::new_end();
		assert_eq!(token.to_string(), ">".to_string());
	}
	#[test]
	fn xml_token_quote() {
		let token = XmlToken::new_quote();
		assert_eq!(token, XmlToken::Quote);
	}
	#[test]
	fn xml_token_quote_to_string() {
		let token = XmlToken::new_quote();
		assert_eq!(token.to_string(), "\"".to_string());
	}
	#[test]
	fn xml_token_value() {
		let token = XmlToken::new_value("value".to_string());
		assert_eq!(token, XmlToken::Value("value".to_string()));
	}
	#[test]
	fn xml_token_value_to_string() {
		let token = XmlToken::new_value("value".to_string());
		assert_eq!(token.to_string(), "value".to_string());
	}
}