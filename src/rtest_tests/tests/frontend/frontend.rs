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
	match Frontend::new() {
		Err(e) => assert!(false, "Failed to initialize frontend: {}", e),
		Ok(frontend) => {
			//Display test: Frontend::log()
			//should display the given message
			println!("Running Frontend::log(); you should see a message below.");
			frontend.log("Test message!");
		}
	}
}

/**
 * Returns all tests in this module.
 */
pub fn all_tests() -> Vec<UnitTest> {
	vec![unit_test!(test_new_frontend), unit_test!(test_frontend_log)]
}
