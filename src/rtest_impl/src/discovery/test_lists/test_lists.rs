/*!
 * Defines the TestLists struct.
 */
use failure::Error;

use super::common;
use super::errors::{TestAddError, TestListIdentifier};
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
	//Private methods.
	/**
	 * TODO
	 */
	fn get_list(&self, list_identifier: TestListIdentifier) -> &Vec<TestEntry> {
		match list_identifier {
			TestListIdentifier::MainList => &self._main_tests,
			TestListIdentifier::ParallelList => &self._parallel_tests,
			TestListIdentifier::IgnoredList => &self._ignored_tests,
		}
	}

	/**
	 * TODO
	 */
	fn get_list_mut(&mut self, list_identifier: TestListIdentifier) -> &mut Vec<TestEntry> {
		match list_identifier {
			TestListIdentifier::MainList => &mut self._main_tests,
			TestListIdentifier::ParallelList => &mut self._parallel_tests,
			TestListIdentifier::IgnoredList => &mut self._ignored_tests,
		}
	}

	//Following two methods are public,
	//as they're used in unit tests.
	/**
	 * If true, the given list contains the given test.
	 */
	pub fn is_in_list(&self, test: &TestEntry, list_identifier: TestListIdentifier) -> bool {
		let list_to_check = self.get_list(list_identifier);

		return list_to_check.contains(test);
	}

	/**
	 * If true, the given list contains more than one instance of the given test.
	 */
	pub fn has_duplicates_in_list(
		&self,
		test: &TestEntry,
		list_identifier: TestListIdentifier,
	) -> bool {
		let list_to_check = self.get_list(list_identifier);

		return list_to_check.iter().filter(|x| **x == *test).count() > 1;
	}

	/**
	 * Checks that the given test was added to the given
	 * list; if not, it raises a debug_assert,
	 * posts an error message to RLS, and returns
	 * an Err(Error). Otherwise returns Ok(()).
	 */
	fn verify_list_append(
		&self,
		test: &TestEntry,
		list_identifier: TestListIdentifier,
	) -> Result<(), Error> {
		if !self.is_in_list(test, list_identifier) {
			let error = TestAddError::ListAppendFailed {
				list: list_identifier,
			};

			debug_assert!(false, "Test {} is not in {} list", test, list_identifier);

			//report to RLS
			common::post_rls_error(&error);
			return Err(error.into());
		}

		Ok(())
	}

	/**
	 * Checks that the given test is not in the given
	 * list; if it is, the function
	 * posts an error message to RLS, and returns
	 * an Err(Error). Otherwise returns Ok(()).
	 */
	fn verify_not_in_list(
		&self,
		test: &TestEntry,
		list_identifier: TestListIdentifier,
	) -> Result<(), Error> {
		if self.is_in_list(test, list_identifier) {
			//If so:
			//	Report error, and return here.
			//	(This is probably being called during macro execution,
			//	so we don't want to halt all other macros)
			let error = TestAddError::TestAlreadyInList {
				list: list_identifier,
			};
			//Report to RLS...
			common::post_rls_error(&error);

			return Err(error.into());
		}

		Ok(())
	}

	/**
	 * TODO
	 */
	fn verify_removed_from_list(
		&self,
		test: &TestEntry,
		list_identifier: TestListIdentifier,
	) -> Result<(), Error>{
		if self.is_in_list(test, list_identifier) {
			let error = TestAddError::ListRemoveFailed {
				list: list_identifier,
			};

			debug_assert!(
				false,
				"Test {} is still in {} list despite being ignored",
				test, list_identifier
			);

			//report to RLS
			common::post_rls_error(&error);

			return Err(error.into());
		}

		Ok(())
	}

	/**
	 * TODO
	 */
	fn add_to_list_and_verify(
		&mut self,
		test: TestEntry,
		list_identifier: TestListIdentifier,
	) -> Result<TestAddError, Error> {
		//Is this already in the desired list?
		if self.is_in_list(&test, list_identifier) {
			//If so:
			//	Early out and report the
			//test's already in the desired list.
			return Ok(TestAddError::TestAlreadyInList {
				list: list_identifier,
			});
		}

		//Add the test
		//self.get_list_mut(list_identifier).(...)
		//presumably you add a clone here
		unimplemented!();

		//DEBUG ASSERT: test really was added
		self.verify_list_append(&test, list_identifier)?;

		Ok(TestAddError::Success)
	}

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
	pub fn add_main_test(&mut self, main_test: TestEntry) -> Result<TestAddError, Error> {
		//Sanity check: Is this already in parallel_tests?
		//Early out if so
		self.verify_not_in_list(&main_test, TestListIdentifier::ParallelList)?;

		//Else, add it to the main list
		self.add_to_list_and_verify(main_test, TestListIdentifier::MainList)
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
	pub fn add_test(&mut self, test: TestEntry) -> Result<TestAddError, Error> {
		//Sanity check: Is this already in main_tests?
		//Early out if so
		self.verify_not_in_list(&test, TestListIdentifier::MainList)?;

		//Else, add it to the parallel list
		self.add_to_list_and_verify(test, TestListIdentifier::ParallelList)
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
	pub fn ignore_test(&mut self, test: TestEntry) -> Result<TestAddError, Error> {
		//Remove test from main_tests and parallel_tests,
		//if it exists in either list.
		//DEBUG ASSERT: There's only one element to be removed.
		unimplemented!();

		//DEBUG ASSERT: test is not in main_tests
		self.verify_removed_from_list(&test, TestListIdentifier::MainList)?;
		//DEBUG ASSERT: test is not in parallel_tests
		self.verify_removed_from_list(&test, TestListIdentifier::ParallelList)?;

		//Now add it to the ignored list
		self.add_to_list_and_verify(test, TestListIdentifier::IgnoredList)
	}
}
