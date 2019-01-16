/*!
 * Defines the Runner struct.
 */
use failure::Error;

use crate::discovery::TestLists;
use crate::frontend::Frontend;
use crate::test_run::TestRunner;

/**
 * When called in the main thread, retrieves all discovered functions and runs them.
 */
pub struct Runner {}

impl Runner {
	pub fn new() -> Runner {
		Runner {}
	}

	/**
	 * Runs the given test list.
	 */
	pub fn run(tests: TestLists) -> Result<(), Error> {
		let mut test_runner = TestRunner::new();
		let frontend = Frontend::new()?;

		//Actually run the tests here...
		let results = test_runner.run(&tests, &frontend);

		match results {
			Ok(_) => {
				//TODO - any reporting here?
				Ok(())
			}
			Err(e) => {
				//TODO - any reporting here?
				Err(e.into())
			}
		}
	}
}
