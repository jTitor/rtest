/*!
 * Defines the test_run module.
 */

mod failure_detail;

mod runner;

/**
 * Returns all tests in this module.
 */
pub fn all_tests() -> Vec<fn()> {
	failure_detail::all_tests().append(runner::all_tests())
}