/*!
 * Defines the RunError enum.
*/
use crate::test_run::RunResults;
use failure::{Error, Fail};

/**
 * If the TestRunner has a critical error stopping it from running all tests, it'll exit with one of this error type,
 * which contains the causing error as well as the RunResults.
 * As the RunResults would be lost on conversion to Error,
 * functions using this should return Result<*, TestRunError> rather than Result<*, Error>.
 */
#[derive(Debug, Fail)]
pub enum TestRunError {
	#[fail(display = "Unknown test run failure")]
	UnknownError,
	#[fail(display = "Not all resources needed for test run could be retrieved: {}", cause)]
	ResourcesNotReady { cause: Error },
	#[fail(display = "Failed to execute all tests in test list: {}, run results before error are: {}", cause, run_results)]
	TestRunError { cause: Error, run_results: RunResults },
}