/*!
 * Defines the TestRunner struct.
*/
use failure::Error;

use super::PrivateImpl;
use crate::discovery::{StaticTestList, StaticTestListInstance};
use crate::frontend::StaticFrontend;
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

		let mut test_list_option: Option<StaticTestListInstance> = None;

		unsafe {
			match StaticTestList::instance() {
				Ok(test_list_result) => {
					test_list_option = Some(test_list_result);

					//The front end will be borrowed per-job,
					//but initialize the instance here if it's not
					//already initialized
					let _frontend_result = StaticFrontend::mut_instance();
				}
				Err(x) => {
					return Err(TestRunError::TestRunError {
						cause: x,
						run_results: run_results,
					});
				}
			}
		}

		if let Some(test_list) = test_list_option {
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
				return Err(TestRunError::TestRunError {
					cause: x,
					run_results: run_results,
				});
			}
		} else {
			//If there's no test list given,
			//report to frontend but return Ok,
			//since the caller had to intentionally
			//send nothing
			unimplemented!();
			return Ok(run_results);
		}

		//Report test success!
		unimplemented!();
		Ok(run_results)
	}
}
