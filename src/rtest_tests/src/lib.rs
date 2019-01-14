/*!
 * Crate root for the library.
 */

//Anything marked "unreachable!()"
//is intentional; it's supposed to indicate
//that the section needs expanding
#![allow(unreachable_code)]

extern crate proc_macro;
extern crate rtest_impl;

#[macro_use]
mod unit_test;

mod discovery;
mod frontend;
mod test_run;

use self::unit_test::UnitTest;

/**
 * Returns all tests in the module.
 */
fn all_tests() -> Vec<UnitTest> {
	let mut result = discovery::all_tests();

	result.append(&mut frontend::all_tests());
	result.append(&mut test_run::all_tests());

	result
}

/**
 * Runs all unit tests in this crate.
 */
pub fn run_unit_tests() -> Result<(), ()> {
	//Get all tests.
	let tests_vec = all_tests();
	let mut pass_count = 0;
	let mut fail_count = 0;

	//For each test:
	for test in tests_vec.iter() {
		//Run it in catch_unwind.
		println!("Running test '{}'", test.name());
		let test_result = test.run();
		match test_result {
			Ok(_) => {
				//If it passed, increment pass count.
				pass_count += 1;
				println!("Test {} passed", test.name());
			}
			Err(fail_reason) => {
				//If it failed, increment fail count.
				fail_count += 1;
				//Report test failed?
				println!("Test {} FAILED: {}", test.name(), fail_reason);
			}
		}
	}

	//Print out pass/fail count.
	println!(
		"{} tests run: {} tests passed, {} tests failed",
		pass_count + fail_count,
		pass_count,
		fail_count
	);

	//If any tests failed, it's an error; else it's ok.
	if fail_count > 0 {
		//Print failure reasons again?
		unimplemented!();

		return Err(());
	}

	Ok(())
}
