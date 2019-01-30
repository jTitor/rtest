/*!
 * Defines the discovery module.
 */
mod static_test_list;

mod test_lists;

mod attributes;

use crate::UnitTest;
/**
 * Returns all tests in this module.
 */
pub fn all_tests() -> Vec<UnitTest> {
	let mut result = static_test_list::all_tests();

	result.append(&mut test_lists::all_tests());
	result.append(&mut attributes::all_tests());

	result
}