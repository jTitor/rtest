/*!
 * Defines the StaticTestList struct.
 */
use std::sync::{Mutex, MutexGuard};

use super::TestLists;

use failure::{Error, Fail};

//TODO
static mut TEST_LISTS: Option<Mutex<TestLists>> = None;

pub type StaticTestListInstance = MutexGuard<'static, TestLists>;
pub type StaticTestListInstanceResult = Result<StaticTestListInstance, Error>;

/**
 * TODO_DESC
 */
#[derive(Debug, Fail)]
pub enum StaticTestListError {
	#[fail(display = "StaticTestList's TestLists not initialized")]
	InstanceNotInitialized,
	#[fail(display = "Lock on StaticTestList's TestLists is poisoned")]
	LockPoisoned,
}

/**
 * TODO_DESC
 */
pub struct StaticTestList {}

impl StaticTestList {
	pub unsafe fn instance() -> StaticTestListInstanceResult {
		if let None = TEST_LISTS {
			TEST_LISTS = Some(Mutex::new(TestLists::new()));
		}
		
		match TEST_LISTS {
			Some(ref lists) => {
				match lists.lock() {
					Ok(lists_read_lock) => { Ok(lists_read_lock) },
					Err(_) => { Err(StaticTestListError::LockPoisoned.into()) }
				}
			}
			_ => { Err(StaticTestListError::InstanceNotInitialized.into()) },
		}
	}
}
