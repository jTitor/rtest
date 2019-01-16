/*!
 * TODO
 */
use rtest_impl::discovery;

use crate::UnitTest;

const TEST_ATTRIBUTE_NAME: &'static str = "random_attribute_name";

fn test_warn_list_instance_failed() {
	//Visibility test: warn_list_instance_failed()
	//should print to terminal/create some sort
	//of warning output
	println!("Calling discovery::warn_list_instance_failed(); you should see a warning message");
	discovery::warn_list_instance_failed(TEST_ATTRIBUTE_NAME);
}

/**
 * Returns all tests in this module.
 */
pub fn all_tests() -> Vec<UnitTest> {
	vec![
		unit_test!(test_warn_list_instance_failed),
	]
}
