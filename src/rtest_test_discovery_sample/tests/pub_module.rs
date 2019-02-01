/*!
 * TODO
 */

#[test]
fn private_fn1() {}

//Functions that have a return value
//should not be valid tests.
#[test]
fn private_fn2_should_not_be_run() -> bool {
	false
}

//Functions that take parameters
//should not be valid tests.
#[test]
fn private_fn3_should_not_be_run(x: bool) {}

#[test]
fn private_fn4_should_not_be_run(x: bool) -> bool {
	false
}

#[ignore]
fn private_ignore_fn1() {}

//As with #[test], functions that have return values
//or parameters should not be marked as ignore,
//since they're not valid tests in the first place.
#[ignore]
fn private_ignore_fn2_should_not_be_traversed() -> bool {
	false
}

#[ignore]
fn private_ignore_fn3_should_not_be_traversed(x: bool) {}

#[ignore]
fn private_ignore_fn4_should_not_be_traversed(x: bool) -> bool {
	false
}

#[test(main)]
fn private_main_fn1() {}

#[test(main)]
fn private_main_fn2_should_not_be_run() -> bool {
	false
}

#[test(main)]
fn private_main_fn3_should_not_be_run(x: bool) {}

#[test(main)]
fn private_main_fn4_should_not_be_run(x: bool) -> bool {
	false
}

//Public functions start here.
#[test]
pub fn public_fn1() {}

#[test]
pub fn public_fn2_should_not_be_run() -> bool {
	false
}

#[test]
pub fn public_fn3_should_not_be_run(x: bool) {}

#[test]
pub fn public_fn4_should_not_be_run(x: bool) -> bool {
	false
}

#[ignore]
pub fn public_ignore_fn1() {}

#[ignore]
pub fn public_ignore_fn2_should_not_be_run() -> bool {
	false
}

#[ignore]
pub fn public_ignore_fn3_should_not_be_run(x: bool) {}

#[ignore]
pub fn public_ignore_fn4_should_not_be_run(x: bool) -> bool {
	false
}

#[test(main)]
pub fn public_main_fn1() {}

#[test(main)]
pub fn public_main_fn2_should_not_be_run() -> bool {
	false
}

#[test(main)]
pub fn public_main_fn3_should_not_be_run(x: bool) {}

#[test(main)]
pub fn public_main_fn4_should_not_be_run(x: bool) -> bool {
	false
}

/**
 * A public struct -
 * no methods in here should be tested or ignored.
 */
pub struct ExampleStruct2 {}
impl ExampleStruct2 {
	#[test]
	pub fn struct_pub_test_fn_should_not_be_run() {}

	#[test]
	fn struct_pri_test_fn1_should_not_be_run() {}

	#[test]
	fn struct_pri_test_fn2_should_not_be_run() -> bool { false }

	#[test]
	fn struct_pri_test_fn3_should_not_be_run(x: bool) -> bool { false }

	#[test]
	pub fn struct_pub_test_method_should_not_be_run(&self) {}

	#[test]
	fn struct_pri_test_method1_should_not_be_run(&self) {}

	#[test]
	fn struct_pri_test_method2_should_not_be_run(&self) -> bool { false }

	#[test]
	fn struct_pri_test_method3_should_not_be_run(&self, x: bool) -> bool { false }

	//
	#[ignore]
	pub fn struct_pub_ignore_fn_should_not_be_run() {}

	#[ignore]
	fn struct_pri_ignore_fn1_should_not_be_run() {}

	#[ignore]
	fn struct_pri_ignore_fn2_should_not_be_run() -> bool { false }

	#[ignore]
	fn struct_pri_ignore_fn3_should_not_be_run(x: bool) -> bool { false }

	#[ignore]
	pub fn struct_pub_ignore_method_should_not_be_run(&self) {}

	#[ignore]
	fn struct_pri_ignore_method1_should_not_be_run(&self) {}

	#[ignore]
	fn struct_pri_ignore_method2_should_not_be_run(&self) -> bool { false }

	#[ignore]
	fn struct_pri_ignore_method3_should_not_be_run(&self, x: bool) -> bool { false }

	//
	#[test(main)]
	pub fn struct_pub_main_fn_should_not_be_run() {}

	#[test(main)]
	fn struct_pri_main_fn1_should_not_be_run() {}

	#[test(main)]
	fn struct_pri_main_fn2_should_not_be_run() -> bool { false }

	#[test(main)]
	fn struct_pri_main_fn3_should_not_be_run(x: bool) -> bool { false }

	#[test(main)]
	pub fn struct_pub_main_method_should_not_be_run(&self) {}

	#[test(main)]
	fn struct_pri_main_method1_should_not_be_run(&self) {}

	#[test(main)]
	fn struct_pri_main_method2_should_not_be_run(&self) -> bool { false }

	#[test(main)]
	fn struct_pri_main_method3_should_not_be_run(&self, x: bool) -> bool { false }
}