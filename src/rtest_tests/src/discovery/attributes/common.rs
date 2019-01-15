/*!
 * TODO
 */
use proc_macro::TokenStream;
use rtest_impl::discovery;

use crate::UnitTest;

fn sample_function() -> TokenStream {
	let expanded = quote! {
		fn example_fn() {
			let mut x = 0;
			x = x + 1;
		}
	};

	expanded.into()
}

fn sample_struct() -> TokenStream {
	let expanded = quote! {
		pub struct ExampleStruct {
			a: i32,
		}
	};

	expanded.into()
}

fn sample_struct_with_method() -> TokenStream {
	let expanded = quote! {
		pub struct ExampleStruct {
			a: i32,
		}

		impl ExampleStruct {
			pub fn example_public_method(&self) -> i32 {
				self.a
			}

			fn example_private_method(&self) {
				let mut x = 0;
				x = x + 1;
			}
		}
	};

	expanded.into()
}

fn sample_type() -> TokenStream {
	let expanded = quote! {
		type ExampleType = i32;
	};

	expanded.into()
}

fn sample_garbage() -> TokenStream {
	let expanded = quote! {
		dcsdioji
	};

	expanded.into()
}

const TEST_ATTRIBUTE_NAME: &'static str = "random_attribute_name";

fn test_is_function() {
	//discovery::is_function(item: &TokenStream)
	//ASSERT: is_function evaluates sample function
	//as true
	let mut desired_result = true;
	assert!(
		discovery::is_function(&sample_function()) == desired_result,
		"discovery::is_function() didn't evaluate function TokenStream as {}",
		desired_result
	);

	//ASSERT: is_function evaluates sample struct
	//as false
	desired_result = false;
	assert!(
		discovery::is_function(&sample_struct()) == desired_result,
		"discovery::is_function() didn't evaluate struct TokenStream as {}",
		desired_result
	);

	//ASSERT: is_function evaluates sample struct
	//with method in impl as false
	assert!(
		discovery::is_function(&sample_struct_with_method()) == desired_result,
		"discovery::is_function() didn't evaluate struct TokenStream with method tokens as {}",
		desired_result
	);

	//ASSERT: is_function evaluates sample type
	//as false
	assert!(
		discovery::is_function(&sample_type()) == desired_result,
		"discovery::is_function() didn't evaluate type TokenStream as {}",
		desired_result
	);

	//ASSERT: is_function evaluates sample garbage
	//as false
	assert!(
		discovery::is_function(&sample_garbage()) == desired_result,
		"discovery::is_function() didn't evaluate garbage TokenStream as {}",
		desired_result
	);
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
