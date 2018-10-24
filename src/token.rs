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
	StartEndElement(String),
	AttributeToken(Attribute<'a>),
	Text(String),
}