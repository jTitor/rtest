/*!
 * Defines the TestRunner struct.
*/
use std::cmp;
use std::panic;
use std::sync::{RwLock, RwLockWriteGuard};

use failure::Error;
use rayon::prelude::*;

use super::{GetLockError, TestRunnerError, TestError};
use super::super::FailureDetail;
use ::src::discovery::{StaticTestList, StaticTestListInstance};
use ::src::frontend::{StaticFrontend, StaticFrontendInstance};

/**
 * Given a TestLists instance, runs the different tests as needed.
 */
pub struct TestRunner {
	_failures: Vec<FailureDetail>,
	_pass_count: u64,
	_fail_count: u64,
	_ignore_count: u64,
	_tests_evaluated_count: u64,
}

/**
 * TODO
 */
fn get_write_lock<'a, T>(lock: &'a RwLock<T>) -> Result<RwLockWriteGuard<'a, T>, Error> {
	let write_lock_result = lock.write();

	match write_lock_result {
		Ok(write_lock) => Ok(write_lock),
		Err(_) => Err(GetLockError::LockPoisoned.into()),
	}
}

/**
 * TODO
 */
fn parallel_job(
	fail_count_lock: RwLock<&mut u64>,
	pass_count_lock: RwLock<&mut u64>,
	tests_evaluated_count_lock: RwLock<&mut u64>,
	test: fn(),
) -> Result<(), Error> {
	//Report that the test is starting to frontend.
	unsafe {
		StaticFrontend::instance()?.log(format!("Starting test: {:?}", test));
	}
	//Run the test.
	let mut test_result = Err(TestError::TestNotRun);

	let initial_test_result = panic::catch_unwind(test);
	match initial_test_result {
		Ok(_) => { test_result = Ok(()); },
		Err(x) => {
			if let Some(fail_string) = x.downcast_ref::<String>() {
				test_result = Err(TestError::TestFailed {
				cause: format!("{}", fail_string) });
			}
			else if let Some(fail_string) = x.downcast_ref::<&str>() {
				test_result = Err(TestError::TestFailed {
				cause: format!("{}", fail_string) });
			}
			else {
				test_result = Err(TestError::TestFailed {
				cause: format!("Couldn't get detailed failure string: {:?}", x) });
			}
		}
	}

	//	Increment run counter accordingly.
	**get_write_lock(&tests_evaluated_count_lock)? += 1;

	//Handle pass/fail...
	let mut frontend_option: Option<StaticFrontendInstance> = None;
	unsafe {
		frontend_option = Some(StaticFrontend::instance()?);
	}
	if let Some(frontend) = frontend_option {
		match test_result {
			Ok(_) => {
				//	Increment pass counter (?) accordingly.
				**get_write_lock(&pass_count_lock)? += 1;
				//Report to frontend.
				frontend.log(format!("Test passed: {:?}", test));
			},
			Err(x) => {
				//increment failure counter accordingly.
				**get_write_lock(&fail_count_lock)? += 1;
				//Report to frontend.
				frontend.log(format!("Test {:?} failed: {}", test, x));
				//Add the failure reason to the overall list.
				x;
				unimplemented!();
			}
		}
	}

	Ok(())
}

impl TestRunner {
	/**
	 * TODO
	 */
	fn handle_ignored_tests(&mut self, test_list: &StaticTestListInstance) -> Result<(), Error> {
		//For each test in ignored_tests:
		for test in test_list.ignored_tests() {
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
	fn run_parallel_tests(&mut self, test_list: &StaticTestListInstance) -> Result<(), Error> {
		//Iterate over parallel_tests:
		//going to do N jobs,
		//where N = number of cores
		let num_cores = 1;
		let num_tests = test_list.tests().len();

		//Setup locks for the jobs
		let pass_count_lock = RwLock::new(&mut self._pass_count);
		let fail_count_lock = RwLock::new(&mut self._fail_count);
		let ignore_count_lock = RwLock::new(&mut self._ignore_count);
		let tests_evaluated_count_lock = RwLock::new(&mut self._tests_evaluated_count);

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

			for y in 0..num_jobs {
				test_failures.push(Ok(()));
			}

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
	fn run_main_tests(&mut self, test_list: &StaticTestListInstance) -> Result<(), Error> {
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
				//	Save any test failure details to self._failures list.
				unimplemented!();
				//	Increment run counter accordingly.
				self._tests_evaluated_count += 1;

				//Handle pass/fail...
				let test_passed = false;
				let test_failed = false;
				if test_failed {
					//increment failure counter accordingly.
					self._fail_count += 1;
					//Report to frontend.
					frontend.log(format!("Test failed: {:?}", test));
				}
				if test_passed {
					//	Increment pass counter (?) accordingly.
					self._pass_count += 1;
					//Report to frontend.
					frontend.log(format!("Test passed: {:?}", test));
				}
			}
		}

		Ok(())
	}

	/**
	 * TODO
	 */
	fn do_run(&mut self) -> Result<(), Error> {
		//Clear all fields.
		self._failures.clear();
		self._pass_count = 0;
		self._fail_count = 0;
		self._ignore_count = 0;
		self._ignore_count = 0;
		self._tests_evaluated_count = 0;

		let mut test_list_option: Option<StaticTestListInstance> = None;

		unsafe {
			let mut test_list_result = StaticTestList::instance()?;

			let test_list_option = Some(test_list_result);

			//The front end will be borrowed per-job,
			//but initialize the instance here if it's not
			//already initialized
			let _frontend_result = StaticFrontend::mut_instance();
		}

		if let Some(test_list) = test_list_option {
			//The order of this may change.
			self.handle_ignored_tests(&test_list)?;

			self.run_parallel_tests(&test_list)?;

			self.run_main_tests(&test_list)?;
		}

		Ok(())
	}

	/**
	 * TODO
	 */
	pub fn run(&mut self) {
		let _ = self.do_run();
	}
}
