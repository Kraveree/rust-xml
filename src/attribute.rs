use std::fmt;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct XmlAttribute {
    /// Attribute name.
    // How to make a restricted string?
    pub name: String,

    /// Attribute value.
    // How to make the value an allowed value?
    pub value: String
}


impl<'a> fmt::Display for XmlAttribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Need to filter out the resistriced values from the value
        write!(f, "{}=\"{}\"", self.name, self.value)
    }
}


impl XmlAttribute {
    /// Creates a borrowed attribute using the provided borrowed name and a borrowed string value.
    #[inline]
    pub fn new(name: String, value: String) -> Option<XmlAttribute> {
        Some(XmlAttribute { name: name, value: value, })
    }
}


#[cfg(test)]
mod tests {
	use super::{XmlAttribute};

	#[test]
	fn new_attribute() {
		let attribute = XmlAttribute::new("name".to_string(), "value".to_string()).unwrap();
		assert_eq!(attribute.name, "name");
		assert_eq!(attribute.value, "value");
	}

	#[test]
	fn display_attribute() {
		let attribute = XmlAttribute::new("name".to_string(), "value".to_string()).unwrap();
		assert_eq!("name=\"value\"".to_owned(), format!("{}", attribute));
	}
}
