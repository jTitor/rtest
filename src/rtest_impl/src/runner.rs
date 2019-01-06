/*!
 * Defines the Runner struct.
 */
use crate::test_run::TestRunner;
use crate::frontend::Frontend

/**
 * When called in the main thread, retrieves all discovered functions and runs them.
 */
pub struct Runner {}

impl Runner {
	pub new() -> Runner { Runner{} }

	pub run(test_functions: Vec<fn()>) {
		let test_runner = TestRunner::new();
		let frontend = Frontend::new();

		//Actually run the tests here...
		unimplemented!();
	}
}