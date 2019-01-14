/*!
 * TODO
 */
use proc_macro::TokenStream;
use rtest_impl::discovery;

use crate::UnitTest;

fn sample_function() -> TokenStream {
	unimplemented!()
}

fn sample_struct() -> TokenStream {
	unimplemented!()
}

fn sample_struct_with_method() -> TokenStream {
	unimplemented!()
}

fn sample_type() -> TokenStream {
	unimplemented!()
}

fn sample_garbage() -> TokenStream {
	unimplemented!()
}

const TEST_ATTRIBUTE_NAME: &'static str = "random_attribute_name";

fn test_is_function() {
	//discovery::is_function(item: &TokenStream)
	//ASSERT: is_function evaluates sample function
	//as true
	let mut desired_result = true;
	assert!(discovery::is_function(&sample_function()) == desired_result, "discovery::is_function() didn't evaluate function TokenStream as {}", desired_result);

	//ASSERT: is_function evaluates sample struct
	//as false
	desired_result = false;
	assert!(discovery::is_function(&sample_struct()) == desired_result, "discovery::is_function() didn't evaluate struct TokenStream as {}", desired_result);

	//ASSERT: is_function evaluates sample struct
	//with method in impl as false
	assert!(discovery::is_function(&sample_struct_with_method()) == desired_result, "discovery::is_function() didn't evaluate struct TokenStream with method tokens as {}", desired_result);

	//ASSERT: is_function evaluates sample type
	//as false
	assert!(discovery::is_function(&sample_type()) == desired_result, "discovery::is_function() didn't evaluate type TokenStream as {}", desired_result);

	//ASSERT: is_function evaluates sample garbage
	//as false
	assert!(discovery::is_function(&sample_garbage()) == desired_result, "discovery::is_function() didn't evaluate garbage TokenStream as {}", desired_result);
}

fn test_warn_not_function() {
	//Visibility test: warn_not_function()
	//should print to terminal/create some sort
	//of warning output
	println!("Calling discovery::warn_not_function(); you should see a warning message");
	discovery::warn_not_function(&sample_garbage(), TEST_ATTRIBUTE_NAME);
}

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
		unit_test!(test_is_function),
		unit_test!(test_warn_not_function),
		unit_test!(test_warn_list_instance_failed),
	]
}
