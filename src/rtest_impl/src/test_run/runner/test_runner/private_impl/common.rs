/*!
 * TODO
*/
use std::panic;
use std::sync::{RwLock, RwLockWriteGuard};

use failure::Error;

use crate::frontend::{StaticFrontend, StaticFrontendInstance};
use crate::test_run::runner::errors::{GetLockError, TestError};

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
	fail_count_lock: RwLock<&mut u64>,
	pass_count_lock: RwLock<&mut u64>,
	tests_evaluated_count_lock: RwLock<&mut u64>,
	test: fn(),
) -> Result<(), Error> {
	//Report that the test is starting to frontend.
	unsafe {
		StaticFrontend::instance()?.log(&format!("Starting test: {:?}", test));
	}
	//Run the test.
	let test_result = do_test(test);

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
				frontend.log(&format!("Test passed: {:?}", test));
			}
			Err(x) => {
				//increment failure counter accordingly.
				**get_write_lock(&fail_count_lock)? += 1;
				//Report to frontend.
				frontend.log(&format!("Test {:?} failed: {}", test, x));
				//Add the failure reason to the overall list.
				x;
				unimplemented!();
			}
		}
	}

	Ok(())
}
