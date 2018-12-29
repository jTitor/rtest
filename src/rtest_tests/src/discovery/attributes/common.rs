/*!
 * TODO
 */
use crate::UnitTest;

fn test_is_function() {
	//ASSERT: is_function evaluates sample function
	//as true
	unimplemented!();

	//ASSERT: is_function evaluates sample struct
	//as false
	unimplemented!();

	//ASSERT: is_function evaluates sample struct
	//with method in impl as false
	unimplemented!();

	//ASSERT: is_function evaluates sample type
	//as false
	unimplemented!();

	//ASSERT: is_function evaluates sample garbage
	//as false
	unimplemented!();
}

fn test_warn_not_function() {
	//Visibility test: warn_not_function()
	//should print to terminal/create some sort
	//of warning output
	unimplemented!();
}

fn test_warn_list_instance_failed() {
	//Visibility test: warn_list_instance_failed()
	//should print to terminal/create some sort
	//of warning output
	unimplemented!();
}

/**
 * Returns all tests in this module.
 */
pub fn all_tests() -> Vec<UnitTest> {
	vec![
		unit_test!(test_is_function),
		unit_test!(test_warn_not_function),
		unit_test!(test_warn_list_instance_failed),
	]
}
