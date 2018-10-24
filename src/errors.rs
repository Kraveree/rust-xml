use std::error;
//use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ExtractError{
	EndTokenMissing(char),
	BeginTokenMissing(char),
}


pub type ExtractResult<T> = Result<T, ExtractError>;


// This is important for other errors to wrap this one.
#[warn(unreachable_patterns)]
impl error::Error for ExtractError {
	fn description(&self) -> &str {
		match self {
			ExtractError::EndTokenMissing(_c) => "Missing end token",
			ExtractError::BeginTokenMissing(_c) => "Missing begin token",
		}
	}

	fn cause(&self) -> Option<&error::Error> {
		// Generic error, underlying cause isn't tracked.
		None
	}
}


impl fmt::Display for ExtractError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			ExtractError::EndTokenMissing(c) => write!(f, "Missing end token {}", c),
			ExtractError::BeginTokenMissing(c) => write!(f, "Missing begin token {}", c),
		}
	}
}


