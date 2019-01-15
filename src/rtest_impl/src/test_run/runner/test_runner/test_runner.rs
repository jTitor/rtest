/*!
 * Defines the TestRunner struct.
*/
use failure::Error;

use super::{PrivateImpl, panic_handling};
use crate::discovery::TestLists;
use crate::frontend::Frontend;
use crate::test_run::{RunResults, TestRunError};

/**
 * Given a TestLists instance, runs the different tests as needed.
 */
pub struct TestRunner {}

impl TestRunner {
	//Private functions.
	/**
	 * Runs the tests supplied to this TestRunner.
	 */
	fn run_tests(
		&mut self,
		//TODO: make this non-mut
		test_list: &TestLists,
		frontend: &Frontend,
	) -> Result<RunResults, TestRunError> {
		let mut run_results = RunResults::new();

		//Run the tests.
		//The order of this may change.
		//TODO: This early aborts
		//if any tests fails???
		//That doesn't sound right
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
			frontend.log("Tests failed!");
			return Err(TestRunError::TestRunError {
				cause: x,
				run_results: run_results,
			});
		}

		//Report test success!
		return Ok(run_results);
	}

	//Public functions.
	/**
	 * Creates a new TestRunner instance.
	 */
	pub fn new() -> TestRunner {
		TestRunner {}
	}

	/**
	 * TODO
	 */
	pub fn run(
		&mut self,
		//TODO: make this non-mut
		test_list: &TestLists,
		frontend: &Frontend,
	) -> Result<RunResults, TestRunError> {
		//Attach the custom panic hook
		//so we get full panic details.
		panic_handling::set_hook();

		//Run the tests...
		let result = self.run_tests(test_list, frontend);

		//Now detach the hook.
		panic_handling::take_hook();

		result
	}
}
