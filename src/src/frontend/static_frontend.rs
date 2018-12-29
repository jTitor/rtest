/*! 
 * Defines the StaticFrontend struct.
 */
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use failure::{Error, Fail};

use super::Frontend;

//TODO
static mut FRONTEND: Option<RwLock<Frontend>> = None;

pub type StaticFrontendInstance = RwLockReadGuard<'static, Frontend>;
pub type StaticFrontendResult = Result<StaticFrontendInstance, Error>;

pub type StaticFrontendMutInstance = RwLockWriteGuard<'static, Frontend>;
pub type StaticFrontendMutResult = Result<StaticFrontendMutInstance, Error>;

/**
 * TODO
 */
#[derive(Debug, Fail)]
pub enum StaticFrontendError {
	#[fail(display = "StaticFrontend's Frontend not initialized")]
	InstanceNotInitialized,
	#[fail(display = "Lock on StaticFrontend's Frontend is poisoned")]
	LockPoisoned,
}

/**
 * Provides static access to the Frontend struct.
 */
pub struct StaticFrontend {}

impl StaticFrontend {
	unsafe fn init_instance() {
		if let None = FRONTEND {
			FRONTEND = Some(RwLock::new(Frontend::new()));
		}

		if let None = FRONTEND {
			debug_assert!(false, "StaticFrontend's Frontend still not valid despite being initialized");
		}
	}

	pub unsafe fn instance() -> StaticFrontendResult {
		StaticFrontend::init_instance();
		
		match FRONTEND {
			Some(ref frontend_instance) => {
				match frontend_instance.read() {
					Ok(frontend_read_lock) => { Ok(frontend_read_lock) },
					Err(_) => { Err(StaticFrontendError::LockPoisoned.into()) }
				}
			}
			_ => { Err(StaticFrontendError::InstanceNotInitialized.into()) },
		}
	}

	pub unsafe fn mut_instance() -> StaticFrontendMutResult {
		StaticFrontend::init_instance();
		
		match FRONTEND {
			Some(ref frontend_instance) => {
				match frontend_instance.write() {
					Ok(frontend_write_lock) => { Ok(frontend_write_lock) },
					Err(_) => { Err(StaticFrontendError::LockPoisoned.into()) }
				}
			}
			_ => { Err(StaticFrontendError::InstanceNotInitialized.into()) },
		}
	}
}
