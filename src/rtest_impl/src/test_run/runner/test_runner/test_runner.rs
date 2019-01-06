/*!
 * Defines the TestRunner struct.
*/
use failure::Error;

use super::PrivateImpl;
use crate::discovery::TestLists;
use crate::frontend::Frontend;
use crate::test_run::{RunResults, TestRunError};

/**
 * Given a TestLists instance, runs the different tests as needed.
 */
pub struct TestRunner {}

impl TestRunner {
	pub fn new() -> TestRunner {
		TestRunner {}
	}

	/**
	 * TODO
	 */
	pub fn do_run(
		&mut self,
		test_list: &TestLists,
		frontend: &Frontend,
	) -> Result<RunResults, TestRunError> {
		let mut run_results = RunResults::new();

		//Run the tests.
		//The order of this may change.
		let test_results: Result<(), Error> = {
			if let Err(x) = self.handle_ignored_tests(&test_list, frontend, &mut run_results) {
				Err(x)
			} else if let Err(x) = self.run_parallel_tests(&test_list, frontend, &mut run_results) {
				Err(x)
			} else if let Err(x) = self.run_main_tests(&test_list, frontend, &mut run_results) {
				Err(x)
			} else {
				Ok(())
			}
		};

		if let Err(x) = test_results {
			//Report that the run failed
			unimplemented!();
			return Err(TestRunError::TestRunError {
				cause: x,
				run_results: run_results,
			});
		}

		//Report test success!
		return Ok(run_results);
	}
}
