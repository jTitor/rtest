/*!
 * Crate root for the library.
 */

//Anything marked "unimplemented!()"
//is intentional; it's supposed to indicate
//that the section needs expanding
#![allow(unreachable_code)]

extern crate proc_macro;
extern crate rtest_impl;

extern crate quote;

#[macro_use]
mod unit_test;

mod common_functions;
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

	let mut failure_reasons_vec: Vec<String> = vec![];

	//For each test:
	for test in tests_vec.iter() {
		//Run it in catch_unwind.
		println!("[rtest_tests] Running test: {}", test.name());
		let test_result = test.run();
		match test_result {
			Ok(_) => {
				//If it passed, increment pass count.
				pass_count += 1;
				println!("[rtest_tests] Test passed: {}", test.name());
			}
			Err(fail_reason) => {
				//If it failed, increment fail count.
				fail_count += 1;

				//Report test failed...
				println!("[rtest_tests] Test FAILED: {}. Reason: {}", test.name(), fail_reason);

				//And add it to the list
				//for the post-run summary.
				failure_reasons_vec.push(format!("{}: {}", test.name(), fail_reason).into());
			}
		}
	}

	//Print out pass/fail count.
	let divider_line = "------------------------";
	println!("\n{}", divider_line);
	println!("Test run complete.");
	if fail_count > 0 {
		println!("Some tests failed!");
	}
	println!(
		"{} tests run: {} tests passed, {} tests failed",
		pass_count + fail_count,
		pass_count,
		fail_count
	);
	println!("{}", divider_line);

	//If any tests failed, it's an error; else it's ok.
	if fail_count > 0 {
		println!("Failure details:");

		//Print failure reasons again?
		for failure in failure_reasons_vec {
			println!("\t{}", failure);
		}

		return Err(());
	}

	Ok(())
}

/**
 * Entry point for the test harness.
 */
fn main() {
	//Run the unit tests; return 1 if any fail,
	//0 if all pass
	::std::process::exit(match run_unit_tests() {
		Ok(_) => 0,
		_ => 1,
	});
}
