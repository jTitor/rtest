/*!
 * TODO
 */
use crate::UnitTest;

fn test_do_attribute_test() {
	//ASSERT: Adding a function with attr: ()
	//succeeds.
	unimplemented!();
	//ASSERT: The test list contains the added function.
	unimplemented!();
	//ASSERT: Adding the function again succeeds.
	unimplemented!();

	//ASSERT: Adding a function with attr: (main)
	//succeeds.
	unimplemented!();
	//ASSERT: The main test list contains the added function.
	unimplemented!();
	//ASSERT: Adding the function again succeeds.
	unimplemented!();

	//ASSERT: Adding a non-function returns Err.
	//(check specific error?)
	unimplemented!();
	//ASSERT: All of the test lists are unchanged.
	unimplemented!();
}

/**
 * Returns all tests in this module.
 */
pub fn all_tests() -> Vec<UnitTest> {
	vec![unit_test!(test_do_attribute_test)]
}