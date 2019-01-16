/*!
 * TODO
 */
use rtest_impl;
use rtest_impl::discovery::TestAddError;
use rtest_impl::discovery::TestEntry;

use crate::common_functions::*;
use crate::UnitTest;

/**
 * Tests that the default instantiation of
 * TestLists works correctly.
 */
fn test_new() {
	//Create a new TestLists.
	let lists = rtest_impl::discovery::TestLists::new();

	//ASSERT: All lists are empty.
	let expected_len = 0;
	let mut actual_len = lists.main_tests().len();
	assert!(
		actual_len < expected_len + 1,
		"New TestLists instance should have {} main tests, has {}",
		expected_len,
		actual_len
	);

	actual_len = lists.tests().len();
	assert!(
		actual_len < expected_len + 1,
		"New TestLists instance should have {} parallel tests, has {}",
		expected_len,
		actual_len
	);

	actual_len = lists.ignored_tests().len();
	assert!(
		actual_len < expected_len + 1,
		"New TestLists instance should have {} ignored tests, has {}",
		expected_len,
		actual_len
	);
}

fn test_modify_lists() {
	//Create a new TestLists.
	let mut lists = rtest_impl::discovery::TestLists::new();

	//Add to each list:
	//1 main test
	//The main test should not be in parallel_tests_to_add
	//or ignored_tests_to_add
	let main_tests_to_add: Vec<TestEntry> = vec![test_entry_named("Main Test 1")];

	//2 parallel tests
	//None of the parallel tests should be in main_tests_to_add
	//or ignored_tests_to_add
	let parallel_tests_to_add: Vec<TestEntry> = vec![
		test_entry_named("Parallel Test 1"),
		test_entry_named("Parallel Test 2"),
		test_entry_named("Parallel Test 3"),
	];

	//3 ignored tests
	//None of the ignored tests should be in main_tests_to_add
	//or parallel_tests_to_add
	let ignored_tests_to_add: Vec<TestEntry> = vec![
		test_entry_named("Ignored Test 1"),
		test_entry_named("Ignored Test 2"),
		test_entry_named("Ignored Test 3"),
	];

	let tests_added = {
		//Actually add the tests,
		//and early out if we encounter a failure.
		let mut result = Ok(());
		for test in main_tests_to_add.iter() {
			if let Err(x) = lists.add_main_test(test.clone()) {
				result = Err(x);
				break;
			}
		}
		if let Ok(()) = result {
			for test in parallel_tests_to_add.iter() {
				if let Err(x) = lists.add_test(test.clone()) {
					result = Err(x);
					break;
				}
			}
			if let Ok(()) = result {
				for test in ignored_tests_to_add.iter() {
					if let Err(x) = lists.ignore_test(test.clone()) {
						result = Err(x);
						break;
					}
				}
			}
		}

		result
	};

	//ASSERT: All add calls passed
	if let Err(x) = tests_added {
		assert!(
			false,
			"Not all add calls on TestEntry passed. Reason: {}",
			x
		);
	}

	//ASSERT: All lists have the expected number of tests.
	let expected_main_len = main_tests_to_add.len();
	let mut actual_len = lists.main_tests().len();
	assert!(
		actual_len == expected_main_len,
		"New TestLists instance should have {} main test(s), has {}",
		expected_main_len,
		actual_len
	);

	let expected_parallel_len = parallel_tests_to_add.len();
	actual_len = lists.tests().len();
	assert!(
		actual_len == expected_parallel_len,
		"New TestLists instance should have {} parallel test(s), has {}",
		expected_parallel_len,
		actual_len
	);

	let expected_ignored_len = ignored_tests_to_add.len();
	actual_len = lists.ignored_tests().len();
	assert!(
		actual_len == expected_ignored_len,
		"New TestLists instance should have {} ignored test(s), has {}",
		expected_ignored_len,
		actual_len
	);

	//ASSERT: All lists have the tests they were given during the add_*() calls,
	//in FIFO order.
	let mut expected_list = &main_tests_to_add;
	let mut actual_list = lists.main_tests();
	assert!(
		*expected_list == *actual_list,
		"Main tests list doesn't match input: input is {:?}, output is {:?}",
		*expected_list,
		*actual_list
	);

	expected_list = &parallel_tests_to_add;
	actual_list = lists.tests();
	assert!(
		*expected_list == *actual_list,
		"Parallel tests list doesn't match input: input is {:?}, output is {:?}",
		*expected_list,
		*actual_list
	);

	expected_list = &ignored_tests_to_add;
	actual_list = lists.ignored_tests();
	assert!(
		*expected_list == *actual_list,
		"Ignored tests list doesn't match input: input is {:?}, output is {:?}",
		*expected_list,
		*actual_list
	);
}

fn test_main_list() {
	//Create a new TestLists.
	let mut lists = rtest_impl::discovery::TestLists::new();

	//Add a main test
	let main_test_to_add = test_entry_named("Main Test 1");
	//Add a parallel test
	let parallel_test_to_add = test_entry_named("Parallel Test 1");
	//Add an ignored test
	let ignored_test_to_add = test_entry_named("Ignored Test 1");

	let add_result = {
		let mut result = Ok(());

		if let Err(x) = lists.add_main_test(main_test_to_add.clone()) {
			result = Err(x);
		} else if let Err(x) = lists.add_test(parallel_test_to_add.clone()) {
			result = Err(x);
		} else if let Err(x) = lists.ignore_test(ignored_test_to_add.clone()) {
			result = Err(x);
		}

		result
	};

	//ASSERT: All add calls passed
	if let Err(x) = add_result {
		assert!(
			false,
			"Not all add calls on TestEntry passed. Reason: {}",
			x
		);
	}

	//ASSERT: Adding a new main test that's not in
	//parallel_tests() or ignored_tests() will succeed
	let next_main_test = test_entry_named("Main Test 2");
	if let Err(x) = lists.add_main_test(next_main_test.clone()) {
		assert!(false, "Adding a unique test to main_tests failed: {}", x);
	}

	//ASSERT: Adding a main test that's already in
	//main_tests() does nothing, but returns Ok() (TODO: "error"s in Ok?)
	let len_before_adding = lists.main_tests().len();
	if let Err(x) = lists.add_main_test(next_main_test.clone()) {
		assert!(
			false,
			"Adding an already added test to main_tests failed: {}",
			x
		);
	}
	let len_after_adding = lists.main_tests().len();
	assert!(
		len_before_adding == len_after_adding,
		"Duplicate test added to main_tests list: list should have {} tests, actually has {}",
		len_before_adding,
		len_after_adding
	);

	//ASSERT: Adding a main test that's already in
	//parallel_tests() returns Err() (TODO: possibly match for "in another list")
	if let Ok(_) = lists.add_main_test(parallel_test_to_add.clone()) {
		assert!(false, "Adding a test that was already in parallel_tests to main_tests should fail but succeeded");
	}

	//ASSERT: Adding a main test that's already in
	//ignored_tests() returns Ok(In another list error).
	let add_ignored_test_to_main = lists.add_main_test(ignored_test_to_add.clone());
	if let Err(_) = add_ignored_test_to_main {
	} else {
		let details_string = {
			match add_ignored_test_to_main {
				Ok(x) => format!("succeeded instead with {}", x),
				Err(e) => format!("failed with {}", e),
			}
		};

		assert!(false, "Adding a test that was already in ignored_tests to main_tests should fail, but {}", details_string);
	}
}

fn test_parallel_list() {
	//Create a new TestLists.
	let mut lists = rtest_impl::discovery::TestLists::new();

	//Add a main test
	let main_test_to_add = test_entry_named("Main Test 1");
	//Add a parallel test
	let parallel_test_to_add = test_entry_named("Parallel Test 1");
	//Add an ignored test
	let ignored_test_to_add = test_entry_named("Ignored Test 1");

	let add_result = {
		let mut result = Ok(());

		if let Err(x) = lists.add_main_test(main_test_to_add.clone()) {
			result = Err(x);
		} else if let Err(x) = lists.add_test(parallel_test_to_add.clone()) {
			result = Err(x);
		} else if let Err(x) = lists.ignore_test(ignored_test_to_add.clone()) {
			result = Err(x);
		}

		result
	};

	//ASSERT: All add calls passed
	if let Err(x) = add_result {
		assert!(
			false,
			"Not all add calls on TestEntry passed. Reason: {}",
			x
		);
	}

	//ASSERT: Adding a new parallel test that's not in
	//main_tests() or ignored_tests() will succeed
	let next_parallel_test = test_entry_named("Parallel Test 2");
	if let Err(x) = lists.add_test(next_parallel_test.clone()) {
		assert!(
			false,
			"Adding a unique test to parallel tests_failed: {}",
			x
		);
	}

	//ASSERT: Adding a parallel test that's already in
	//parallel_tests() does nothing, but returns Ok() (TODO: "error"s in Ok?)
	let len_before_adding = lists.tests().len();
	if let Err(x) = lists.add_test(next_parallel_test.clone()) {
		assert!(
			false,
			"Adding an already added test to parallel_tests failed: {}",
			x
		);
	}
	let len_after_adding = lists.tests().len();
	assert!(
		len_before_adding == len_after_adding,
		"Duplicate test added to parallel_tests list: list should have {} tests, actually has {}",
		len_before_adding,
		len_after_adding
	);

	//ASSERT: Adding a parallel test that's already in
	//main_tests() returns Err() (TODO: possibly match for "in another list")
	if let Ok(_) = lists.add_test(main_test_to_add.clone()) {
		assert!(false, "Adding a test that was already in main_tests to parallel_tests should fail but succeeded");
	}

	//ASSERT: Adding a parallel test that's already in
	//ignored_tests() returns Ok(In another list error).
	let add_ignored_test_to_parallel = lists.add_test(ignored_test_to_add.clone());
	if let Err(_) = add_ignored_test_to_parallel {
	} else {
		let details_string = {
			match add_ignored_test_to_parallel {
				Ok(x) => format!("succeeded instead with {}", x),
				Err(e) => format!("failed with {}", e),
			}
		};

		assert!(false, "Adding a test that was already in ignored_tests to parallel_tests should fail, but {}", details_string);
	}
}

fn test_ignored_list() {
	//Create a new TestLists.
	let mut lists = rtest_impl::discovery::TestLists::new();

	//Add a main test
	let main_test_to_add = test_entry_named("Main Test 1");
	//Add a parallel test
	let parallel_test_to_add = test_entry_named("Parallel Test 1");
	//Add an ignored test
	let ignored_test_to_add = test_entry_named("Ignored Test 1");

	let add_result = {
		let mut result = Ok(());

		if let Err(x) = lists.add_main_test(main_test_to_add.clone()) {
			result = Err(x);
		} else if let Err(x) = lists.add_test(parallel_test_to_add.clone()) {
			result = Err(x);
		} else if let Err(x) = lists.ignore_test(ignored_test_to_add.clone()) {
			result = Err(x);
		}

		result
	};

	//ASSERT: All add calls passed
	if let Err(x) = add_result {
		assert!(
			false,
			"Not all add calls on TestEntry passed. Reason: {}",
			x
		);
	}

	//ASSERT: Ignoring a new test that's not in
	//parallel_tests() or main_tests() will succeed
	let next_ignored_test = test_entry_named("Ignored Test 2");
	if let Err(x) = lists.ignore_test(next_ignored_test.clone()) {
		assert!(
			false,
			"Adding a unique test to parallel_tests failed: {}",
			x
		);
	}

	//ASSERT: Ignoring an already ignored test
	//does nothing, but returns Ok() (TODO: "error"s in Ok?)
	let len_before_adding = lists.ignored_tests().len();
	if let Err(x) = lists.ignore_test(next_ignored_test.clone()) {
		assert!(
			false,
			"Adding an already added test to ignored_tests failed: {}",
			x
		);
	}
	let len_after_adding = lists.ignored_tests().len();
	assert!(
		len_before_adding == len_after_adding,
		"Duplicate test added to ignored_tests list: list should have {} tests, actually has {}",
		len_before_adding,
		len_after_adding
	);

	//ASSERT: Ignoring a main test succeeds
	if let Err(x) = lists.ignore_test(main_test_to_add.clone()) {
		assert!(false, "Ignoring a test in main_tests failed: {}", x);
	}
	//ASSERT: The ignored main test is no longer in main_tests()
	if let Some(_) = lists.main_tests().iter().find(|&x| x == &main_test_to_add) {
		assert!(
			false,
			"Test that should be removed from main_tests is still there"
		);
	}

	//ASSERT: Ignoring a parallel test succeeds
	if let Err(x) = lists.ignore_test(parallel_test_to_add.clone()) {
		assert!(
			false,
			"Ignoring a test in parallel tests list failed: {}",
			x
		);
	}
	//ASSERT: The ignored parallel test is no longer in tests()
	if let Some(_) = lists.tests().iter().find(|&x| x == &parallel_test_to_add) {
		assert!(
			false,
			"Test that should be removed from parallel tests list is still there"
		);
	}
}

/**
 * Returns all tests in this module.
 */
pub fn all_tests() -> Vec<UnitTest> {
	vec![
		unit_test!(test_new),
		unit_test!(test_modify_lists),
		unit_test!(test_main_list),
		unit_test!(test_parallel_list),
		unit_test!(test_ignored_list),
	]
}
