/*!
 * TODO
 */
use crate::UnitTest;

fn test_frontend_instance() {
	//Get the instance.
	unimplemented!();

	//ASSERT: Instance is non-Err.
	unimplemented!();
}

fn test_frontend_mut_instance() {
	//Get the instance.
	unimplemented!();

	//ASSERT: Instance is non-Err.
	unimplemented!();
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
