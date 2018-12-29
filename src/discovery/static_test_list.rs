/*!
 * Defines the StaticTestList struct.
 */
use std::sync::{Mutex, MutexGuard};

use super::TestLists;

use failure::{Error, Fail};

//TODO
static mut test_lists: Option<Mutex<TestLists>> = None;

/**
 * TODO
 */
#[derive(Debug, Fail)]
pub enum StaticTestListError {
	#[fail(display = "StaticTestList's TestLists not initialized")]
	InstanceNotInitialized,
	#[fail(display = "Lock on StaticTestList's TestLists is poisoned")]
	LockPoisoned,
}

/**
 * TODO
 */
pub struct StaticTestList {}

impl StaticTestList {
	pub unsafe fn instance() -> Result<MutexGuard<'static, TestLists>, Error> {
		if let None = test_lists {
			test_lists = Some(Mutex::new(TestLists::new()));
		}
		
		match test_lists {
			Some(ref lists) => {
				match lists.lock() {
					Ok(lists_read_lock) => { Ok(lists_read_lock) },
					Err(x) => { Err(StaticTestListError::LockPoisoned.into()) }
				}
			}
			_ => { Err(StaticTestListError::InstanceNotInitialized.into()) },
		}
	}
}
