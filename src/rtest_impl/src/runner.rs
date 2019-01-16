/*!
 * Defines the Runner struct.
 */
use crate::test_run::TestRunner;
use crate::frontend::Frontend;
use crate::discovery::TestLists;

/**
 * When called in the main thread, retrieves all discovered functions and runs them.
 */
pub struct Runner {}

impl Runner {
	pub fn new() -> Runner { Runner{} }

	/**
	 * Runs the given test list.
	 */
	pub fn run(tests: TestLists) -> Result<(), ()> {
		let mut test_runner = TestRunner::new();
		let frontend = Frontend::new();

		//Actually run the tests here...
		let results = test_runner.run(&tests, &frontend);
		
		match results {
			Ok(_) => {
				//TODO - any reporting here?
				Ok(()) }
			Err(_) => {
				//TODO - any reporting here?
				Err(())
			}
		}
	}
}