/*!
 * Defines errors used by this module.
 */
use std::fmt;

use failure::Fail;

const PARALLEL_TESTS_LIST_NAME: &'static str = "main list";
const MAIN_TESTS_LIST_NAME: &'static str = "parallel list";
const IGNORED_TESTS_LIST_NAME: &'static str = "ignored list";

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
			MainList => display_str = MAIN_TESTS_LIST_NAME,
			ParallelList => display_str = PARALLEL_TESTS_LIST_NAME,
			IgnoredList => display_str = IGNORED_TESTS_LIST_NAME,
		}

		write!(f, "{}", display_str)
	}
}

/**
 * TODO
 */
#[derive(Debug, Fail)]
pub enum TestAddError {
	#[fail(display = "successfully added test")]
	Success,
	#[fail(display = "{} already contains test", list)]
	TestAlreadyInList { list: TestListIdentifier },
	#[fail(display = "Test could not be added to {} for unknown reason", list)]
	ListAppendFailed { list: TestListIdentifier },
	#[fail(display = "Test could not be removed from {} for unknown reason", list)]
	ListRemoveFailed { list: TestListIdentifier },
}
