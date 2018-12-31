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
pub fn parallel_job(
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
		Ok(_) => {
			test_result = Ok(());
		}
		Err(x) => {
			if let Some(fail_string) = x.downcast_ref::<String>() {
				test_result = Err(TestError::TestFailed {
					cause: format!("{}", fail_string),
				});
			} else if let Some(fail_string) = x.downcast_ref::<&str>() {
				test_result = Err(TestError::TestFailed {
					cause: format!("{}", fail_string),
				});
			} else {
				test_result = Err(TestError::TestFailed {
					cause: format!("Couldn't get detailed failure string: {:?}", x),
				});
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
			}
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