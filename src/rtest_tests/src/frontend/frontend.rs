/*!
 * TODO
 */
use crate::UnitTest;

fn test_new_frontend() {
	unimplemented!();
}

fn test_frontend_log() {
	unimplemented!();
}

/**
 * Returns all tests in this module.
 */
pub fn all_tests() -> Vec<UnitTest> {
	vec![unit_test!(test_new_frontend), unit_test!(test_frontend_log)]
}
