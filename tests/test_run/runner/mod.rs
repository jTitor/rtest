/*!
 * Defines the runner module.
 */

mod runner;

/**
 * Returns all tests in this module.
 */
pub fn all_tests() -> Vec<fn()> {
	runner::all_tests()
}