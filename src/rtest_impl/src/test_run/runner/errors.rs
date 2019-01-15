/*!
 * Defines errors used in the runner module.
 */
use failure::{Error, Fail};

#[derive(Debug, Fail)]
pub enum TestRunnerError {
	#[fail(display = "Failed to evaluate all ignored tests: {}", cause)]
	FailedAtIgnoreStage { cause: Error },
	#[fail(display = "Failed to evaluate all parallel tests: {}", cause)]
	FailedAtParallelStage { cause: Error },
	#[fail(display = "A parallel job failed: {}", cause)]
	JobFailed { cause: Error },
	#[fail(display = "A parallel job had an unexpected panic: {}", cause)]
	JobUnexpected { cause: Error },
	#[fail(display = "Failed to evaluate all main-thread tests {}", cause)]
	FailedAtMainThreadStage { cause: Error },
	#[fail(display = "Unknown failure")]
	UnknownError,
}

#[derive(Debug, Fail)]
pub enum GetLockError {
	#[fail(display = "A lock was poisoned")]
	LockPoisoned,
}

#[derive(Debug, Fail)]
pub enum TestError {
	#[fail(display = "{}", cause)]
	TestFailed { cause: String },
	#[fail(display = "Test not run")]
	TestNotRun,
}