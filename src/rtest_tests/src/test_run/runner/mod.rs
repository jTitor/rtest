/*!
 * Defines the runner module.
 */
mod runner;

use crate::UnitTest;

/**
 * Returns all tests in this module.
 */
pub fn all_tests() -> Vec<UnitTest> {
	runner::all_tests()
}