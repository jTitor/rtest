/*!
 * TODO
 */
use rtest_impl::discovery::StaticTestList;

use crate::UnitTest;

fn test_instance() {
	unsafe {
		//Get the instance.
		if let Err(e) = StaticTestList::instance() {
			//ASSERT: Instance is non-Err.
			assert!(false, "StaticTestList::instance() should return a TestLists instance, returned error {}", e);
		}
	}
}

/**
 * Returns all tests in this module.
 */
pub fn all_tests() -> Vec<UnitTest> {
	vec![unit_test!(test_instance)]
}