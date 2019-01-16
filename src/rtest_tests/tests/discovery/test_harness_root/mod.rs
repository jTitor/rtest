/*!
 * Defines the test_harness_root module.
 */

mod test_harness_root;

use crate::UnitTest;

/**
 * Returns all tests in this module.
 */
pub fn all_tests() -> Vec<UnitTest> {
	let result = test_harness_root::all_tests();

	result
}