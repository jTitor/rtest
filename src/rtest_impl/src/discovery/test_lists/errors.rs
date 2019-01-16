/*!
 * Defines errors used by this module.
 */
use std::fmt;

use failure::Fail;

const PARALLEL_TESTS_LIST_NAME: &'static str = "main list";
const MAIN_TESTS_LIST_NAME: &'static str = "parallel list";
const IGNORED_TESTS_LIST_NAME: &'static str = "ignored list";

pub fn list_element_name() -> String {
	"test".into()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TestListIdentifier {
	MainList,
	ParallelList,
	IgnoredList,
}

impl fmt::Display for TestListIdentifier {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut display_str = "";

		match self {
			TestListIdentifier::MainList => display_str = MAIN_TESTS_LIST_NAME,
			TestListIdentifier::ParallelList => display_str = PARALLEL_TESTS_LIST_NAME,
			TestListIdentifier::IgnoredList => display_str = IGNORED_TESTS_LIST_NAME,
		}

		write!(f, "{}", display_str)
	}
}
