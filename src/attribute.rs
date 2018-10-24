use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Attribute<'a> {
    /// Attribute name.
    // How to make a restricted string?
    pub name: &'a str,

    /// Attribute value.
    // How to make the value an allowed value?
    pub value: &'a str
}


impl<'a> fmt::Display for Attribute<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Need to filter out the resistriced values from the value
        write!(f, "{}=\"{}\"", self.name, self.value)
    }
}


impl<'a> Attribute<'a> {
    /// Creates a borrowed attribute using the provided borrowed name and a borrowed string value.
    #[inline]
    pub fn new(name: &'a str, value: &'a str) -> Attribute<'a> {
        Attribute { name, value, }
    }
}


#[cfg(test)]
mod tests {
	use super::{Attribute};

	#[test]
	fn new_attribute() {
		let attribute = Attribute::new("name", "value");
		assert_eq!(attribute.name, "name");
		assert_eq!(attribute.value, "value");
	}

	#[test]
	fn display_attribute() {
		let attribute = Attribute::new("name", "value");
		assert_eq!("name=\"value\"".to_owned(), format!("{}", attribute));
	}
}
