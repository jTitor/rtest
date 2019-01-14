/*!
 * TODO
 */
use crate::UnitTest;

fn test_new_frontend() {
	//ASSERT: Frontend::new()
	//returns a Frontend instance
	unimplemented!();
}

fn test_frontend_log() {
	//Display test: Frontend::log()
	//should display the given message
	unimplemented!();
}

/**
 * Returns all tests in this module.
 */
pub fn all_tests() -> Vec<UnitTest> {
	vec![unit_test!(test_new_frontend), unit_test!(test_frontend_log)]
}
