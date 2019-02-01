/*!
 * TODO
 */

#[test]
fn private_fn1() {}

//Functions that have a return value
//should not be valid tests.
//(rustc gives a weird error about this
//before proc expansion, remarking that
//main() can't return a value,
//so comment this out)
// #[test]
// fn private_fn2_should_not_be_run() -> bool {
// 	false
// }

//Functions that take parameters
//should not be valid tests.
//(rustc flags these as errors before proc
//expansion even happens, so it's ok)
// #[test]
// fn private_fn3_should_not_be_run(x: bool) {}

// #[test]
// fn private_fn4_should_not_be_run(x: bool) -> bool {
// 	false
// }

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

// #[test(main)]
// fn private_main_fn2_should_not_be_run() -> bool {
// 	false
// }

// #[test(main)]
// fn private_main_fn3_should_not_be_run(x: bool) {}

// #[test(main)]
// fn private_main_fn4_should_not_be_run(x: bool) -> bool {
// 	false
// }

//Public functions start here.
#[test]
pub fn public_fn1() {}

// #[test]
// pub fn public_fn2_should_not_be_run() -> bool {
// 	false
// }

// #[test]
// pub fn public_fn3_should_not_be_run(x: bool) {}

// #[test]
// pub fn public_fn4_should_not_be_run(x: bool) -> bool {
// 	false
// }

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

// #[test(main)]
// pub fn public_main_fn2_should_not_be_run() -> bool {
// 	false
// }

// #[test(main)]
// pub fn public_main_fn3_should_not_be_run(x: bool) {}

// #[test(main)]
// pub fn public_main_fn4_should_not_be_run(x: bool) -> bool {
// 	false
// }

/**
 * A public struct -
 * methods in here should not be tested.
 */
pub struct ExampleStruct2 {}
impl ExampleStruct2 {
	//As with tests taking parameters,
	//rustc flags an error when
	//struct class/instance methods are marked
	//with #[test], so comment these out too
	// #[test]
	// pub fn struct_pub_test_fn_should_not_be_run() {}

	// #[test]
	// fn struct_pri_test_fn1_should_not_be_run() {}

	// #[test]
	// fn struct_pri_test_fn2_should_not_be_run() -> bool { false }

	// #[test]
	// fn struct_pri_test_fn3_should_not_be_run(x: bool) -> bool { false }

	// #[test]
	// pub fn struct_pub_test_method_should_not_be_run(&self) {}

	// #[test]
	// fn struct_pri_test_method1_should_not_be_run(&self) {}

	// #[test]
	// fn struct_pri_test_method2_should_not_be_run(&self) -> bool { false }

	// #[test]
	// fn struct_pri_test_method3_should_not_be_run(&self, x: bool) -> bool { false }

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
	// #[test(main)]
	// pub fn struct_pub_main_fn_should_not_be_run() {}

	// #[test(main)]
	// fn struct_pri_main_fn1_should_not_be_run() {}

	// #[test(main)]
	// fn struct_pri_main_fn2_should_not_be_run() -> bool { false }

	// #[test(main)]
	// fn struct_pri_main_fn3_should_not_be_run(x: bool) -> bool { false }

	// #[test(main)]
	// pub fn struct_pub_main_method_should_not_be_run(&self) {}

	// #[test(main)]
	// fn struct_pri_main_method1_should_not_be_run(&self) {}

	// #[test(main)]
	// fn struct_pri_main_method2_should_not_be_run(&self) -> bool { false }

	// #[test(main)]
	// fn struct_pri_main_method3_should_not_be_run(&self, x: bool) -> bool { false }
}