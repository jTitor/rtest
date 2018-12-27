/**
 * Defines the TestLists struct.
 */

/**
 * Contains all of the tests marked by the #[test] and #[ignore] tags.
 */
pub struct TestLists {
	_main_tests: Vec<u64>,
	_parallel_tests: Vec<u64>,
	_ignored_tests: Vec<u64>,
}

impl TestLists {
	/**
	 * Creates a new TestLists instance.
	 */
	pub fn new() -> TestLists {
		TestLists {
			_main_tests: vec!(),
			_parallel_tests: vec!(),
			_ignored_tests: vec!(),
		}
	}

	//List operations.
	//There aren't any clear or remove operations, since there's no #[remove_test] attribute.

	/**
	 * Returns the list of main-thread-only tests recorded in this TestLists instance.
	 */
	pub fn main_tests(&self) -> &Vec<u64> {
		&self._main_tests
	}

	/**
	 * Adds a test to the TestList's list of main-thread-only tests.
	 */
	pub fn add_main_test(&mut self, main_test: u64) {
		//Sanity check: Is this already in parallel_tests?
		//If so:
		//	Report error, and return here.
		//	(This is probably being called during macro execution,
		//	so we don't want to halt all other macros)

		//Else:
		//	Add the test to main_tests.

		//DEBUG ASSERT: test is now in main_tests
		unimplemented!()
	}

	/**
	 * Returns the list of parallelizable tests recorded in this TestLists instance.
	 */
	pub fn tests(&self) -> &Vec<u64> {
		&self._parallel_tests
	}

	/**
	 * Adds a test to the TestList's list of parallelizable tests.
	 */
	pub fn add_test(&mut self, test: u64) {
		//Sanity check: Is this already in main_tests?
		//If so:
		//	Report error, and return here.
		//	(This is probably being called during macro execution,
		//	so we don't want to halt all other macros)

		//Else:
		//	Add the test to parallel_tests.

		//DEBUG ASSERT: test is now in parallel_tests
		unimplemented!()
	}

	/**
	 * Returns the list of ignored tests recorded in this TestLists instance.
	 */
	pub fn ignored_tests(&self) -> &Vec<u64> {
		&self._ignored_tests
	}

	/**
	 * Marks a test as an ignored test in the TestList.
	 */
	pub fn ignore_test(&mut self, test: u64) {
		//Remove test from main_tests and parallel_tests,
		//if it exists in either list.

		//DEBUG ASSERT: test is not in main_tests
		//DEBUG ASSERT: test is not in parallel_tests

		//Add the test to ignored_tests.

		//DEBUG ASSERT: test is now in ignored_tests
		unimplemented!()
	}
}