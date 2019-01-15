/*!
 * TODO
 */
use rtest_impl::frontend::Frontend;

use crate::UnitTest;

fn test_new_frontend() {
	//ASSERT: Frontend::new()
	//returns a Frontend instance
	let _ = Frontend::new();
}

fn test_frontend_log() {
	//Display test: Frontend::log()
	//should display the given message
	println!("Running Frontend::log(); you should see a message below.");
	Frontend::new().log("Test message!");
}

/**
 * Returns all tests in this module.
 */
pub fn all_tests() -> Vec<UnitTest> {
	vec![unit_test!(test_new_frontend), unit_test!(test_frontend_log)]
}
