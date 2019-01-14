/*!
 * TODO
 */
use rtest_impl::frontend::StaticFrontend;

use crate::UnitTest;

fn test_frontend_instance() {
	unsafe {
		//Get the instance.
		if let Err(e) = StaticFrontend::instance() {
			//ASSERT: Instance is non-Err.
			assert!(false, "StaticFrontend::instance() should return a TestLists instance, returned error {}", e);
		}
	}
}

fn test_frontend_mut_instance() {
	unsafe {
		//Get the instance.
		if let Err(e) = StaticFrontend::mut_instance() {
			//ASSERT: Instance is non-Err.
			assert!(false, "StaticFrontend::mut_instance() should return a TestLists instance, returned error {}", e);
		}
	}
}

/**
 * Returns all tests in this module.
 */
pub fn all_tests() -> Vec<UnitTest> {
	vec![
		unit_test!(test_frontend_instance),
		unit_test!(test_frontend_mut_instance),
	]
}
