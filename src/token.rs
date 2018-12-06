use attribute::Attribute;

#[derive(Debug, Copy, Clone, PartialEq)] 
pub enum WhitespaceKind {
	Space(usize),
	Tab(usize),
	CR(usize),
	LF(usize),
	FF(usize),
}


#[derive(Debug, Copy, Clone, PartialEq)] 
pub enum SymbolKind {
	Symbol(char),
}


#[derive(Debug, Clone, PartialEq)] 
pub enum XmlToken<'a> {
	Whitespace(WhitespaceKind),
	StartElement(String),
	EndElement(String),
	AttributeToken(Attribute<'a>),
	Text(String),
	Other(String),
}

trait Value {
	fn to_string(self) -> Option<String>;
}

impl<'a> Value for XmlToken<'a> {
	fn to_string(self) -> Option<String> {
		match self {
			XmlToken::StartElement(s) => Some(s),
			XmlToken::EndElement(s) => Some(s),
			XmlToken::Text(s) => Some(s),
			XmlToken::Other(s) => Some(s),
			_ => None
		}
	}
}