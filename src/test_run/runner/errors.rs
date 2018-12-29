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
}

#[derive(Debug, Fail)]
pub enum GetLockError {
	#[fail(display = "A lock was poisoned")]
	LockPoisoned,
}