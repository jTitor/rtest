/*!
 * TODO
 */
use rtest_impl::discovery::{TestEntry, TestLists};
use rtest_impl::frontend::Frontend;
use rtest_impl::test_run::TestRunner;

use crate::common_functions::test_entry_named;
use crate::UnitTest;

/**
 * A function that will fail a test.
 */
fn fail_fn() {
	assert!(false, "intentionally failing for fail_fn()");
}

//Test list initializers.
/**
 * Makes a TestLists instance where
 * all tests pass.
 */
fn make_test_list_all_pass() -> TestLists {
	let mut result = TestLists::new();
	let _ = result.add_main_test(test_entry_named("mt1"));
	let _ = result.add_test(test_entry_named("pt1"));
	
	result
}

/**
 * Makes a TestLists instance where
 * all tests are skipped.
 */
fn make_test_list_all_skip() -> TestLists {
	let mut result = TestLists::new();
	let _ = result.ignore_test(test_entry_named("it1"));
	
	result
}

/**
 * Makes a TestLists instance where
 * all tests fail.
 */
fn make_test_list_all_fail() -> TestLists {
	let mut result = TestLists::new();
	let fail_test = TestEntry {
		name: "fp1".into(),
		test: fail_fn
	};
	let fail_main_test = TestEntry {
		name: "fm1".into(),
		test: fail_fn
	};

	let _ = result.add_main_test(fail_main_test);
	let _ = result.add_test(fail_test);
	
	result
}

/**
 * Makes a TestLists instance where some
 * but not all tests fail.
 * Some tests are also skipped.
 */
fn make_test_list_some_fail() -> TestLists {
	let mut result = TestLists::new();
	let fail_test = TestEntry {
		name: "fp1".into(),
		test: fail_fn
	};
	
	//3 tests should pass
	let _ = result.add_test(test_entry_named("pt1"));
	let _ = result.add_main_test(test_entry_named("mt1"));
	let _ = result.add_test(test_entry_named("pt2"));

	//2 tests should skip
	let _ = result.ignore_test(test_entry_named("it1"));
	let _ = result.ignore_test(test_entry_named("it2"));

	//1 test should fail
	let _ = result.add_test(fail_test);

	result
}

//Unit tests.
fn test_runner_counts() {
	//Setup constants.
	let expected_pass_count = 3;
	let expected_skip_count = 2;
	let expected_fail_count = 1;

	//Init runner.
	let mut runner = TestRunner::new();
	let frontend = Frontend::new();

	//Run test list.
	let test_some_fail = make_test_list_some_fail();

	match runner.run(&test_some_fail, &frontend) {
		Ok(run_results) => {
			//ASSERT: passed test count matches expected count.
			let actual_count = run_results.pass_count();
			assert!(
				actual_count == expected_pass_count,
				"Pass count should be {}, is {}",
				expected_pass_count,
				actual_count
			);
			//ASSERT: failed test count matches expected count.
			let actual_count = run_results.failure_count();
			assert!(
				actual_count == expected_fail_count,
				"Fail count should be {}, is {}",
				expected_fail_count,
				actual_count
			);
			//ASSERT: ignored test count matches expected count.
			let actual_count = run_results.ignore_count();
			assert!(
				actual_count == expected_skip_count,
				"Skip count should be {}, is {}",
				expected_skip_count,
				actual_count
			);

			//ASSERT: total test count matches expected count,
			//the sum of the previous expected counts.
			let actual_sum = run_results.tests_evaluated_count();
			let expected_sum = expected_pass_count + expected_fail_count + expected_skip_count;
			assert!(
				actual_sum == expected_sum,
				"Total test count should be {}, is {}",
				expected_sum,
				actual_sum
			);

			//ASSERT: number of test failure details equals
			//failed test count.
			let actual_count = run_results.failure_details().len();
			assert!(
				actual_count == expected_fail_count,
				"Number of failure detail entries should be {}, is {}",
				expected_fail_count,
				actual_count
			);
		}
		Err(e) => {
			assert!(false, "Failed to run all tests: {}", e);
		}
	}
}

fn test_runner_result() {
	//Init runner.
	let mut runner = TestRunner::new();
	let frontend = Frontend::new();

	//ASSERT: runner returns Ok() when
	//all tests pass.
	let test_all_pass = make_test_list_all_pass();
	if let Err(e) = runner.run(&test_all_pass, &frontend) {
		assert!(
			false,
			"Runner is supposed to return Ok() when all tests pass, returned Err instead: {}",
			e
		);
	}

	//ASSERT: runner returns Ok() when
	//all tests are skipped.
	let test_all_skip = make_test_list_all_skip();
	if let Err(e) = runner.run(&test_all_skip, &frontend) {
		assert!(false, "Runner is supposed to return Ok() when all tests are skipped, returned Err instead: {}", e);
	}

	//ASSERT: runner returns Err() when
	//any test fails.
	let test_some_fail = make_test_list_some_fail();
	if let Err(e) = runner.run(&test_some_fail, &frontend) {
		assert!(
			false,
			"Runner is supposed to return Ok() if all tests were run even if when any test fails, returned Err() instead: {}",
			e
		);
	}

	//ASSERT: runner returns Err() when
	//all tests fail.
	let test_all_fail = make_test_list_all_fail();
	if let Err(e) = runner.run(&test_all_fail, &frontend) {
		assert!(
			false,
			"Runner is supposed to return Ok() if all tests were run even if all tests fail, returned Err() instead: {}",
			e
		);
	}
}

/**
 * Returns all tests in this module.
 */
pub fn all_tests() -> Vec<UnitTest> {
	vec![
		unit_test!(test_runner_counts),
		unit_test!(test_runner_result),
	]
}
