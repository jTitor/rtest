/*!
 * Defines the test_run module.
 */
mod failure_detail;

mod runner;

use crate::UnitTest;

/**
 * Returns all tests in this module.
 */
pub fn all_tests() -> Vec<UnitTest> {
	let mut result = failure_detail::all_tests();

	result.append(&mut runner::all_tests());

	result
}