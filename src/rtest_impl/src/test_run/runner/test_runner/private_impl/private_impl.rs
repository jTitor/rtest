/*!
 * Private implementation details of TestRunner.
*/
use std::cmp;
use std::panic;
use std::sync::RwLock;

use failure::Error;
use rayon::prelude::*;

use super::common::*;
use crate::discovery::StaticTestListInstance;
use crate::frontend::{StaticFrontend, StaticFrontendInstance};
use crate::test_run::runner::errors::TestRunnerError;
use crate::test_run::FailureDetail;
use crate::test_run::{RunResults, TestRunner};

pub trait PrivateImpl {
	fn handle_ignored_tests(
		&mut self,
		test_list: &StaticTestListInstance,
		run_results: &mut RunResults,
	) -> Result<(), Error>;

	fn run_parallel_tests(
		&mut self,
		test_list: &StaticTestListInstance,
		run_results: &mut RunResults,
	) -> Result<(), Error>;

	fn run_main_tests(
		&mut self,
		test_list: &StaticTestListInstance,
		run_results: &mut RunResults,
	) -> Result<(), Error>;
}

impl PrivateImpl for TestRunner {
	/**
	 * TODO
	 */
	fn handle_ignored_tests(
		&mut self,
		test_list: &StaticTestListInstance,
		run_results: &mut RunResults,
	) -> Result<(), Error> {
		//For each test in ignored_tests:
		for test in test_list.ignored_tests().iter() {
			//	Report that the test is ignored to frontend.
			unimplemented!();
			//	Increment ignored counter accordingly.
			unimplemented!();
		}

		Ok(())
	}

	/**
	 * TODO
	 */
	fn run_parallel_tests(
		&mut self,
		test_list: &StaticTestListInstance,
		run_results: &mut RunResults,
	) -> Result<(), Error> {
		//Iterate over parallel_tests:
		//going to do N jobs,
		//where N = number of cores
		let num_cores = 1;
		let num_tests = test_list.tests().len();

		//Setup locks for the jobs
		let pass_count_lock = RwLock::new(&mut run_results.pass_count);
		let fail_count_lock = RwLock::new(&mut run_results.fail_count);
		let ignore_count_lock = RwLock::new(&mut run_results.ignore_count);
		let tests_evaluated_count_lock = RwLock::new(&mut run_results.tests_evaluated_count);

		for x in 0..(num_tests / num_cores) {
			let base_index = x * num_cores;
			//Sanity check: base index <= number of tests,
			//otherwise our clamping won't make sense
			assert!(base_index <= num_tests);

			//Going to make N jobs,
			//where N = number of cores on system
			//Clamp to remaining number of tests
			let num_jobs = cmp::min(num_cores, num_tests - base_index);
			//	Create a list of N test failure details
			let mut test_failures: Vec<Result<(), TestRunnerError>> = vec![];

			for _ in 0..num_jobs {
				test_failures.push(Ok(()));
			}

			//Dispatch the N jobs:
			for y in 0..num_jobs {
				//	Get next N tests
				let test_index = base_index + y;
				unimplemented!();

				//	Create a job for each test:
				let job = |test: fn()| {
					//Actually run the test.
					let job_result = parallel_job(
						fail_count_lock,
						pass_count_lock,
						tests_evaluated_count_lock,
						test,
					);

					//Convert any errors into our error type.
					test_failures[y] = {
						match job_result {
							Ok(_) => Ok(()),
							Err(x) => Err(TestRunnerError::JobFailed { cause: x.into() }),
						}
					};
				};
				//	Dispatch the job...
				unimplemented!();
			}

			//Wait for the N jobs.
			for y in 0..num_jobs {
				//Now wait for each job.
				unimplemented!();
				//If a job had an error, copy the error details here.
				unimplemented!();
			}
		}

		Ok(())
	}

	/**
	 * TODO
	 */
	fn run_main_tests(
		&mut self,
		test_list: &StaticTestListInstance,
		run_results: &mut RunResults,
	) -> Result<(), Error> {
		//For each test in main_tests:
		for test in test_list.main_tests().iter() {
			//	Report that the test is starting to frontend.
			let mut frontend_option: Option<StaticFrontendInstance> = None;
			unsafe {
				let frontend = StaticFrontend::instance().unwrap();
				frontend_option = Some(frontend);
			}

			if let Some(frontend) = frontend_option {
				frontend.log(format!("Test starting: {:?}", test));
				//	Run the test.
				unimplemented!();
				//	Save any test failure details to run_results.failures list.
				unimplemented!();
				//	Increment run counter accordingly.
				run_results.tests_evaluated_count += 1;

				//Handle pass/fail...
				let test_passed = false;
				let test_failed = false;
				if test_failed {
					//increment failure counter accordingly.
					run_results.fail_count += 1;
					//Report to frontend.
					frontend.log(format!("Test failed: {:?}", test));
				}
				if test_passed {
					//	Increment pass counter (?) accordingly.
					run_results.pass_count += 1;
					//Report to frontend.
					frontend.log(format!("Test passed: {:?}", test));
				}
			}
		}

		Ok(())
	}
}
