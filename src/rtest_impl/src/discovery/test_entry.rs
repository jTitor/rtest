/*!
 * TODO
*/
use std::fmt;

/**
 * TODO_DESC
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestEntry {
	pub name: String,
	pub test: fn(),
}

impl TestEntry {
	/**
	 * Generates a TestEntry vector from a vector of names
	 * and a vector of test functions.
	 */
	pub fn vec_from_vecs(names: Vec<String>, tests: Vec<fn()>) -> Vec<TestEntry> {
		//Return empty list if lists aren't the same size
		if names.len() != tests.len() {
			return vec![];
		}
		
		names.into_iter().zip(tests.into_iter())
		.map(|(n, t)| {
			TestEntry { name: n, test: t }
		})
		.collect::<Vec<_>>()
	}
}

impl fmt::Display for TestEntry {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.name)
	}
}
