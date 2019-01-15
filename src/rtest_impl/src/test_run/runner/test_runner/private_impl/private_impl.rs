/*!
 * Private implementation details of TestRunner.
*/
use std::cmp;
use std::panic;
use std::sync::{Mutex, RwLock};

use failure::Error;
use rayon::prelude::*;

use super::common::*;
use crate::discovery::TestLists;
use crate::frontend::Frontend;
use crate::test_run::runner::errors::TestRunnerError;
use crate::test_run::FailureDetail;
use crate::test_run::{RunResults, TestRunner};

pub trait PrivateImpl {
	/**
	 * Runs all ignored tests
	 * (that is, marks all ignored tests as skipped).
	 *
	 * Returns an Err(Error) only if a test
	 * can't be marked as skipped somehow.
	 */
	fn handle_ignored_tests(
		&mut self,
		test_list: &TestLists,
		frontend: &Frontend,
		run_results: &mut RunResults,
	) -> Result<(), Error>;

	/**
	 * Runs all non-main-thread tests.
	 *
	 * Returns Ok() if all tests were run,
	 * regardless of if a test failed.
	 */
	fn run_parallel_tests(
		&mut self,
		test_list: &TestLists,
		frontend: &Frontend,
		run_results: &mut RunResults,
	) -> Result<(), Error>;

	/**
	 * Runs all main-thread tests.
	 *
	 * Returns Ok() if all tests were run,
	 * regardless of if a test failed.
	 */
	fn run_main_tests(
		&mut self,
		test_list: &TestLists,
		frontend: &Frontend,
		run_results: &mut RunResults,
	) -> Result<(), Error>;
}

impl PrivateImpl for TestRunner {
	fn handle_ignored_tests(
		&mut self,
		test_list: &TestLists,
		frontend: &Frontend,
		run_results: &mut RunResults,
	) -> Result<(), Error> {
		//For each test in ignored_tests:
		for test in test_list.ignored_tests().iter() {
			//	Report that the test is ignored to frontend.
			frontend.log(&format!("Ignoring test: {}", test));
			//	Increment ignored counter accordingly.
			run_results.add_ignore();
		}

		Ok(())
	}

	fn run_parallel_tests(
		&mut self,
		test_list: &TestLists,
		frontend: &Frontend,
		run_results: &mut RunResults,
	) -> Result<(), Error> {
		//Setup locks for the jobs
		let run_result_lock = RwLock::new(run_results);
		let frontend_mutex = Mutex::new(frontend);
		let mut any_jobs_failed = false;
		let any_jobs_failed_lock = Mutex::new(&mut any_jobs_failed);

		//Dispatch the tests in parallel...
		//Any test failures will have already been added
		//to run_results, so drop the result collection
		//we get from the tests
		let _ = test_list
			.tests()
			.par_iter()
			.map(|test_entry| {
				//Actually run the test.
				parallel_job(&run_result_lock, &frontend_mutex, test_entry.test)
			})
			.collect::<Result<(), Error>>();

		Ok(())
	}

	fn run_main_tests(
		&mut self,
		test_list: &TestLists,
		frontend: &Frontend,
		run_results: &mut RunResults,
	) -> Result<(), Error> {
		//For each test in main_tests:
		for test in test_list.main_tests().iter() {
			//	Report that the test is starting to frontend.
			frontend.log(&format!("Test starting: {}", test));

			//	Run the test.
			let test_result = test_on_main_thread(test.test);

			//Handle pass/fail...
			//an Err() here indicates the test failed,
			//whether the code being tested failed
			//or the harness itself.
			if let Err(test_error) = test_result {
				//	Save any test failure details to run_results.failures list.
				let failure_detail =
					FailureDetail::parse_from_string(&test.to_string(), &test_error.to_string());

				run_results.add_failure(failure_detail);

				//Report to frontend.
				frontend.log(&format!("Test failed: {:?}", test));
			}

			//Otherwise, report a pass
			let test_passed = false;
			if test_passed {
				//	Increment pass counter (?) accordingly.
				run_results.add_pass();
				//Report to frontend.
				frontend.log(&format!("Test passed: {:?}", test));
			}
		}

		Ok(())
	}
}
