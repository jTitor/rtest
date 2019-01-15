/*!
 * TODO
*/
use std::panic;
use std::sync::{Mutex, MutexGuard, RwLock, RwLockWriteGuard};

#[cfg(macos)]
use dispatch::Queue;
use failure::Error;

use crate::discovery::TestEntry;
use crate::frontend::Frontend;
use crate::test_run::runner::errors::{GetLockError, TestError};
use crate::test_run::{FailureDetail, RunResults};

/**
 * TODO
 */
pub fn get_mutex_lock<'a, T>(lock: &'a Mutex<T>) -> Result<MutexGuard<'a, T>, Error> {
	let lock_result = lock.lock();

	match lock_result {
		Ok(return_lock) => Ok(return_lock),
		Err(_) => Err(GetLockError::LockPoisoned.into()),
	}
}

/**
 * TODO
 */
pub fn get_write_lock<'a, T>(lock: &'a RwLock<T>) -> Result<RwLockWriteGuard<'a, T>, Error> {
	let write_lock_result = lock.write();

	match write_lock_result {
		Ok(write_lock) => Ok(write_lock),
		Err(_) => Err(GetLockError::LockPoisoned.into()),
	}
}

/**
 * TODO
 */
pub fn do_test(test: fn()) -> Result<(), TestError> {
	let initial_test_result = panic::catch_unwind(test);
	match initial_test_result {
		Ok(_) => {
			return Ok(());
		}
		Err(x) => {
			if let Some(fail_string) = x.downcast_ref::<String>() {
				return Err(TestError::TestFailed {
					cause: format!("{}", fail_string),
				});
			} else if let Some(fail_string) = x.downcast_ref::<&str>() {
				return Err(TestError::TestFailed {
					cause: format!("{}", fail_string),
				});
			} else {
				return Err(TestError::TestFailed {
					cause: format!("Couldn't get detailed failure string: {:?}", x),
				});
			}
		}
		_ => {
			return Err(TestError::TestFailed {
				cause: "Test failed for unknown reason".to_string(),
			});
		}
	}
}

/**
 * TODO
 */
pub fn parallel_job(
	run_results_lock: &RwLock<&mut RunResults>,
	frontend_mutex: &Mutex<&Frontend>,
	test: &TestEntry,
) -> Result<(), Error> {
	//Report that the test is starting to frontend.
	get_mutex_lock(frontend_mutex)?.log(&format!("Starting test: {}", test));

	//Run the test.
	let test_result = do_test(test.test);

	//Handle pass/fail...
	match test_result {
		Ok(_) => {
			//	Increment pass counter (?) accordingly.
			get_write_lock(run_results_lock)?.add_pass();
			//Report to frontend.
			get_mutex_lock(frontend_mutex)?.log(&format!("Test passed: {}", test));
		}
		Err(x) => {
			let error_string: String = format!("{}", x).to_string();
			//Add the failure reason to the overall list.
			//increment failure counter accordingly.
			get_write_lock(run_results_lock)?.add_failure(FailureDetail::parse_from_string(
				"TODO",
				error_string.as_str(),
			));
			//Report to frontend.
			get_mutex_lock(frontend_mutex)?
				.log(&format!("Test failed: {}. Reason: {}", test, error_string));
		}
	}

	Ok(())
}

fn run_on_main_thread(
	function: fn(fn()) -> Result<(), TestError>,
	test_function: fn(),
) -> Result<(), TestError> {
	//Depends on OS.

	//If on macOS, use GCD.
	#[cfg(macos)]
	{
		return Queue::main().sync(|| {
			return function(test_function);
		});
	}
	//Under anything else, run the function directly
	//as the runner has to operate under the main thread.
	#[cfg(not(macos))]
	{
		return function(test_function);
	}

	unreachable!();
}

pub fn test_on_main_thread(test_function: fn()) -> Result<(), TestError> {
	run_on_main_thread(do_test, test_function)
}
