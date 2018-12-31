/*!
 * TODO
*/
use std::fmt;

/**
 * TODO
*/
#[derive(Debug, Clone)]
pub struct TestEntry {
	pub name: String,
	pub test: fn(),
}

impl fmt::Display for TestEntry {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.name)
	}
}
