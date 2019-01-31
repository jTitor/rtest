/*!
 * Defines the TestLists struct.
 */
use failure::Error;

use super::errors::{TestListIdentifier, list_element_name};
use crate::discovery::rls_common;
use crate::discovery::TestEntry;
use crate::errors::ElementAddError;

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
			let error = ElementAddError::ListAppendFailed {
				element_type: list_element_name(),
				list_name: format!("{}", list_identifier)
			};

			debug_assert!(false, "Test {} is not in {} list", test, list_identifier);

			//report to RLS
			rls_common::post_rls_error(&error);
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
			let error = ElementAddError::AlreadyInList {
				element_type: list_element_name(),
				list_name: format!("{}", list_identifier)
			};
			//Report to RLS...
			rls_common::post_rls_error(&error);

			return Err(error.into());
		}

		Ok(())
	}

	/**
	 * Removes all instances of the given
	 * test from the requested test list.
	 *
	 * Returns Ok if the item was successfully removed,
	 * Err otherwise.
	 */
	fn remove_from_list(
		&mut self,
		test: &TestEntry,
		list_identifier: TestListIdentifier,
	) -> Result<(), Error> {
		//Get indices of all instances.
		let mut found_instance_indices: Vec<usize> = vec![];
		{
			let list = self.get_list(list_identifier);
			for i in 0..list.len() {
				if list[i] == *test {
					//Add indices to the front
					//so when we call remove()
					//it doesn't disturb indices of
					//any other instances we found
					found_instance_indices.insert(i, 0);
				}
			}
		}

		//Now actually remove the instances...
		for index in found_instance_indices {
			self.get_list_mut(list_identifier).remove(index);
		}

		Ok(())
	}

	/**
	 * Verifies that the given TestEntry is not in the specified list.
	 *
	 * Returns Ok if the TestEntry was verified as
	 * not being in the specified list,
	 * Err otherwise.
	 */
	fn verify_removed_from_list(
		&self,
		test: &TestEntry,
		list_identifier: TestListIdentifier,
	) -> Result<(), Error> {
		if self.is_in_list(test, list_identifier) {
			let error = ElementAddError::ListRemoveFailed {
				element_type: list_element_name(),
				list_name: format!("{}", list_identifier)
			};

			debug_assert!(
				false,
				"Test {} is still in {} list despite being ignored",
				test, list_identifier
			);

			//report to RLS
			rls_common::post_rls_error(&error);

			return Err(error.into());
		}

		Ok(())
	}

	/**
	 * Adds the given TestEntry to the specified list, then verifies that
	 * it actually is in that list.
	 *
	 * Returns Ok if the TestEntry was verified as being in the specified list,
	 * Err otherwise.
	 */
	fn add_to_list_and_verify(
		&mut self,
		test: TestEntry,
		list_identifier: TestListIdentifier,
	) -> Result<ElementAddError, Error> {
		//Is this already in the desired list?
		if self.is_in_list(&test, list_identifier) {
			//If so:
			//	Early out and report the
			//test's already in the desired list.
			return Ok(ElementAddError::AlreadyInList {
				element_type: list_element_name(),
				list_name: format!("{}", list_identifier)
			});
		}

		//Add the test
		self.get_list_mut(list_identifier).push(test.clone());

		//DEBUG ASSERT: test really was added
		self.verify_list_append(&test, list_identifier)?;

		Ok(ElementAddError::Success { element_type: list_element_name() })
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
	 *
	 * Returns Ok if the test could be added to the list or was already in the list,
	 * and Err otherwise.
	 */
	pub fn add_main_test(&mut self, main_test: TestEntry) -> Result<ElementAddError, Error> {
		//Sanity check: Is this already in parallel_tests?
		//Early out if so
		self.verify_not_in_list(&main_test, TestListIdentifier::ParallelList)?;
		//Similarly, early out if already in ignored_tests
		self.verify_not_in_list(&main_test, TestListIdentifier::IgnoredList)?;

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
	 *
	 * Returns Ok if the test could be added to the list or was already in the list,
	 * and Err otherwise.
	 */
	pub fn add_test(&mut self, test: TestEntry) -> Result<ElementAddError, Error> {
		//Sanity check: Is this already in main_tests?
		//Early out if so
		self.verify_not_in_list(&test, TestListIdentifier::MainList)?;
		//Similarly, early out if already in ignored_tests
		self.verify_not_in_list(&test, TestListIdentifier::IgnoredList)?;

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
	 *
	 * Returns Ok if the test could be added to the ignore list or was already in the list,
	 * and Err otherwise.
	 */
	pub fn ignore_test(&mut self, test: TestEntry) -> Result<ElementAddError, Error> {
		//Remove test from main_tests and parallel_tests,
		//if it exists in either list.
		self.remove_from_list(&test, TestListIdentifier::MainList)?;
		self.remove_from_list(&test, TestListIdentifier::ParallelList)?;

		//DEBUG ASSERT: test is not in main_tests
		self.verify_removed_from_list(&test, TestListIdentifier::MainList)?;
		//DEBUG ASSERT: test is not in parallel_tests
		self.verify_removed_from_list(&test, TestListIdentifier::ParallelList)?;

		//Now add it to the ignored list
		self.add_to_list_and_verify(test, TestListIdentifier::IgnoredList)
	}

	/**
	 * Appends the given TestEntry's parallel, main, and ignored
	 * test lists to this TestEntry.
	 */
	pub fn push(&mut self, other: &TestLists) {
		//TODO: really don't like this, but
		//both TestLists should be valid so their combined list
		//should still be valid
		self._parallel_tests.clone_from(other.tests());
		self._main_tests.clone_from(other.main_tests());
		self._ignored_tests.clone_from(other.ignored_tests());
	}

	/**
	 * Appends each of the given TestEntry instances to this TestEntry.
	 */
	pub fn append(&mut self, others: &Vec<TestLists>) {
		for list in others {
			self.push(list);
		}
	}

	/**
	 * Generates a TestList
	 * from the given lists of TestEntries.
	 */
	pub fn from_test_entries(
		parallel_tests: Vec<TestEntry>,
		main_tests: Vec<TestEntry>,
		ignored_tests: Vec<TestEntry>,
	) -> Result<TestLists, Error> {
		let mut result = TestLists::new();

		for test in parallel_tests.into_iter() {
			result.add_test(test)?;
		}

		for test in main_tests.into_iter() {
			result.add_main_test(test)?;
		}

		for test in ignored_tests.into_iter() {
			result.ignore_test(test)?;
		}

		Ok(result)
	}
}
