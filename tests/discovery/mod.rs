/*!
 * Defines the discovery module.
 */
mod static_test_list;

mod test_lists;

mod attributes;

/**
 * Returns all tests in this module.
 */
pub fn all_tests() -> Vec<fn()> {
	static_test_list::all_tests().append(test_lists::all_tests().append(attributes::all_tests()))
}