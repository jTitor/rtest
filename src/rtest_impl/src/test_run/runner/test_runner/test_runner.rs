/*!
 * Defines the TestRunner struct.
*/
use failure::Error;

use super::PrivateImpl;
use crate::discovery::{StaticTestList, StaticTestListInstance};
use crate::frontend::{StaticFrontend, StaticFrontendInstance};
use crate::test_run::{FailureDetail, RunResults, TestRunError};

/**
 * Given a TestLists instance, runs the different tests as needed.
 */
pub struct TestRunner {}

impl TestRunner {
	/**
	 * TODO
	 */
	fn do_run(&mut self) -> Result<RunResults, TestRunError> {
		let mut run_results = RunResults::new();

		//TODO: parameterize this and frontend
		let mut test_list_option: Option<StaticTestListInstance> = None;

		let mut frontend_option: Option<StaticFrontendInstance> = None;

		unsafe {
			match (StaticTestList::instance(), StaticFrontend::instance()) {
				(Ok(test_list_result), Ok(frontend_result)) => {
					test_list_option = Some(test_list_result);

					//The front end will be borrowed per-job,
					//but initialize the instance here if it's not
					//already initialized
					frontend_option = Some(frontend_result);
				}
				(Err(x), _) | (_, Err(x)) => {
					//Early out with error if either couldn't be retrieved
					return Err(TestRunError::ResourcesNotReady { cause: x });
				}
			}
		}

		if let (Some(test_list), Some(frontend)) = (test_list_option, frontend_option) {
			//Run the tests.
			//The order of this may change.
			let test_results: Result<(), Error> = {
				if let Err(x) = self.handle_ignored_tests(&test_list, &mut run_results) {
					Err(x)
				} else if let Err(x) = self.run_parallel_tests(&test_list, &mut run_results) {
					Err(x)
				} else if let Err(x) = self.run_main_tests(&test_list, &mut run_results) {
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
		} else {
			//Not all resources are available,
			//so we can't report failure to frontend
			//Just return an error here
			return Err(TestRunError::ResourcesNotReady {
				cause: TestRunError::UnknownError.into(),
			});
		}
		unreachable!();
	}
}
