	use std::str::Chars;

	trait Extractor {
		fn peek_first(&mut self) -> Option<char>;
	}


	impl <'a> Extractor for Chars<'a> {
		fn peek_first(&mut self) -> Option<char> {
			self.peekable().peek().cloned()
		}
	}


	#[cfg(test)]
	mod tests {
		use std::str::Chars;
		use super::Extractor;

		#[test]
		fn peek_first_test() {
			let mut iterator: Chars;
			iterator = "".chars();
			assert_eq!(iterator.peek_first(), None);
			assert_eq!(iterator.as_str(), "".to_string());

			iterator = "A".chars();
			assert_eq!(iterator.peek_first(), Some('A'));
			assert_eq!(iterator.as_str(), "".to_string());

			iterator = "AB".chars();
			assert_eq!(iterator.peek_first(), Some('A'));
			assert_eq!(iterator.as_str(), "B".to_string());
		}
	} 

