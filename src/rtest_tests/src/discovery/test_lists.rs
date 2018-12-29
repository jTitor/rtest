/*!
 * TODO
 */
use crate::UnitTest;

use rtest_impl;

fn test_new() {
	//Create a new TestLists.
	let lists = rtest_impl::discovery::TestLists::new();

	//ASSERT: All lists are empty.
	let expected_len = 0;
	let mut actual_len = lists.main_tests().len();
	assert!(actual_len < expected_len + 1, "New TestLists instance should have {} main tests, has {}", expected_len, actual_len);

	actual_len = lists.tests().len();
	assert!(actual_len < expected_len + 1, "New TestLists instance should have {} parallel tests, has {}", expected_len, actual_len);

	actual_len = lists.ignored_tests().len();
	assert!(actual_len < expected_len + 1, "New TestLists instance should have {} ignored tests, has {}", expected_len, actual_len);
}

fn test_modify_lists() {
	//Create a new TestLists.
	let mut lists = rtest_impl::discovery::TestLists::new();

	//Add to each list:
	//1 main test
	//The main test should not be in parallel_tests_to_add
	//or ignored_tests_to_add
	let main_tests_to_add: Vec<u64> = vec![1];
	for test in main_tests_to_add.iter() {
		lists.add_main_test(*test);
	}
	//2 parallel tests
	//None of the parallel tests should be in main_tests_to_add
	//or ignored_tests_to_add
	let parallel_tests_to_add: Vec<u64> = vec![2, 3, 4];
	for test in parallel_tests_to_add.iter() {
		lists.add_test(*test);
	}
	//3 ignored tests
	//None of the ignored tests should be in main_tests_to_add
	//or parallel_tests_to_add
	let ignored_tests_to_add: Vec<u64> = vec![5, 6, 7];
	for test in ignored_tests_to_add.iter() {
		lists.ignore_test(*test);
	}

	//ASSERT: All lists have the expected number of tests.
	let expected_main_len = main_tests_to_add.len();
	let mut actual_len = lists.main_tests().len();
	assert!(actual_len == expected_main_len, "New TestLists instance should have {} main test(s), has {}", expected_main_len, actual_len);

	let expected_parallel_len = parallel_tests_to_add.len();
	actual_len = lists.tests().len();
	assert!(actual_len == expected_parallel_len + 1, "New TestLists instance should have {} parallel test(s), has {}", expected_parallel_len, actual_len);

	let expected_ignored_len = ignored_tests_to_add.len();
	actual_len = lists.ignored_tests().len();
	assert!(actual_len == expected_ignored_len + 1, "New TestLists instance should have {} ignored test(s), has {}", expected_ignored_len, actual_len);

	//ASSERT: All lists have the tests they were given during the add_*() calls,
	//in FIFO order.
	let mut expected_list = &main_tests_to_add;
	let mut actual_list = lists.main_tests();
	assert!(*expected_list == *actual_list, "Main tests list doesn't match input: input is {:?}, output is {:?}", *expected_list, *actual_list);

	expected_list = &parallel_tests_to_add;
	actual_list = lists.tests();
	assert!(*expected_list == *actual_list, "Parallel tests list doesn't match input: input is {:?}, output is {:?}", *expected_list, *actual_list);

	expected_list = &ignored_tests_to_add;
	actual_list = lists.ignored_tests();
	assert!(*expected_list == *actual_list, "Ignored tests list doesn't match input: input is {:?}, output is {:?}", *expected_list, *actual_list);
}

/**
 * Returns all tests in this module.
 */
pub fn all_tests() -> Vec<UnitTest> {
	vec![unit_test!(test_new), unit_test!(test_modify_lists)]
}