/*!
 * Defines the TestLists struct.
 */
use crate::discovery::TestEntry;

/**
 * Contains all of the tests marked by the #[test] and #[ignore] tags.
 */
pub struct TestLists {
	_main_tests: Vec<TestEntry>,
	_parallel_tests: Vec<TestEntry>,
	_ignored_tests: Vec<TestEntry>,
}

impl TestLists {
	/**
	 * Creates a new TestLists instance.
	 */
	pub fn new() -> TestLists {
		TestLists {
			_main_tests: vec![],
			_parallel_tests: vec![],
			_ignored_tests: vec![],
		}
	}

	//List operations.
	//There aren't any clear or remove operations, since there's no #[remove_test] attribute.

	/**
	 * Returns the list of main-thread-only tests recorded in this TestLists instance.
	 */
	pub fn main_tests(&self) -> &Vec<TestEntry> {
		&self._main_tests
	}

	/**
	 * Adds a test to the TestList's list of main-thread-only tests.
	 */
	pub fn add_main_test(&mut self, main_test: TestEntry) {
		//Sanity check: Is this already in parallel_tests?
		let is_in_parallel_tests = true;
		if is_in_parallel_tests {
			//If so:
			//	Report error, and return here.
			//	(This is probably being called during macro execution,
			//	so we don't want to halt all other macros)
			let error_message = format!(
				"Can't add test {} to main test list: test is already in parallel tests list",
				main_test
			);
			unimplemented!();

			return;
		}
		//Else:
		else {
			//	Add the test to main_tests.
			unimplemented!();
		}

		//DEBUG ASSERT: test is now in main_tests
		let is_in_main_tests = false;
		unimplemented!();
		debug_assert!(
			is_in_main_tests,
			"Test {} is not in main tests list",
			main_test
		);
	}

	/**
	 * Returns the list of parallelizable tests recorded in this TestLists instance.
	 */
	pub fn tests(&self) -> &Vec<TestEntry> {
		&self._parallel_tests
	}

	/**
	 * Adds a test to the TestList's list of parallelizable tests.
	 */
	pub fn add_test(&mut self, test: TestEntry) {
		//Sanity check: Is this already in main_tests?
		let is_in_main_tests = true;
		if is_in_main_tests {
			//If so:
			//	Report error, and return here.
			//	(This is probably being called during macro execution,
			//	so we don't want to halt all other macros)
			let error_message = format!(
				"Can't add test {} to parallel test list: test is already in main tests list",
				test
			);
			unimplemented!();

			return;
		}
		//Else:
		else {
			//	Add the test to parallel_tests.
			unimplemented!();
		}

		//DEBUG ASSERT: test is now in parallel_tests
		let is_in_parallel_tests = false;
		unimplemented!();
		debug_assert!(
			is_in_parallel_tests,
			"Test {} is not in parallel tests list",
			test
		);
	}

	/**
	 * Returns the list of ignored tests recorded in this TestLists instance.
	 */
	pub fn ignored_tests(&self) -> &Vec<TestEntry> {
		&self._ignored_tests
	}

	/**
	 * Marks a test as an ignored test in the TestList.
	 */
	pub fn ignore_test(&mut self, test: TestEntry) {
		//Remove test from main_tests and parallel_tests,
		//if it exists in either list.
		unimplemented!();

		//DEBUG ASSERT: test is not in main_tests
		let is_in_main_tests = true;
		unimplemented!();
		debug_assert!(
			!is_in_main_tests,
			"Test {} is still in main tests list despite being ignored",
			test
		);
		//DEBUG ASSERT: test is not in parallel_tests
		let is_in_parallel_tests = true;
		unimplemented!();
		debug_assert!(
			!is_in_parallel_tests,
			"Test {} is still in parallel tests list despite being ignored",
			test
		);

		//Add the test to ignored_tests.
		unimplemented!();

		//DEBUG ASSERT: test is now in ignored_tests
		let is_in_ignored_tests = false;
		unimplemented!();
		debug_assert!(
			is_in_ignored_tests,
			"Test {} is not in ignored tests list",
			test
		);
	}
}
