/*!
 * TODO
 */
use rtest_impl::discovery::{TestEntry, TestLists};

use crate::UnitTest;
use super::example_functions;
use super::super::common_functions;


fn setup_test_lists() -> TestLists {
	let mut result = TestLists::new();
	//Add main and parallel functions.
	// result.add_main_test(TestEntry {
	// 	name: "main_fn_1",
	// 	test:
	// });
	unimplemented!();

	result
}

//TODO: Is it actually possible to run these tests?
//These take TokenStreams
//but must somehow get fn() pointers from the
//TokenStream to create their TestEntry
fn test_ignore_new_function() {
	let test_lists = setup_test_lists();

	//Test ignoring a new function...
	//ASSERT: Ignoring a newly-seen function succeeds.
	unimplemented!();
	//ASSERT: The ignore list contains the ignored function.
	unimplemented!();
	//ASSERT: The ignore list has only grown by 1.
	unimplemented!();
	//ASSERT: The ignore list's other contents are the same.
	unimplemented!();
	//ASSERT: The main list does not contain the ignored function.
	unimplemented!();
	//ASSERT: The main list's content is unchanged.
	unimplemented!();
	//ASSERT: The parallel list does not contain the ignored function.
	unimplemented!();
	//ASSERT: The parallel list's content is unchanged.
	unimplemented!();

	//ASSERT: Ignoring the function again succeeds.
	unimplemented!();
	//ASSERT: All lists are unchanged.
	unimplemented!();
}

fn test_ignore_main_function() {
	let test_lists = setup_test_lists();

	//Test ignoring a main function...
	//ASSERT: Ignoring a main function succeeds.
	unimplemented!();
	//ASSERT: The ignore list contains the ignored function.
	unimplemented!();
	//ASSERT: The ignore list has only grown by 1.
	unimplemented!();
	//ASSERT: The ignore list's other contents are the same.
	unimplemented!();
	//ASSERT: The main list does not contain the ignored function.
	unimplemented!();
	//ASSERT: The main list does not contain the ignored function.
	unimplemented!();
	//ASSERT: The main list has only shrunk by 1.
	unimplemented!();
	//ASSERT: The main list's other contents are the same.
	unimplemented!();
	//ASSERT: The parallel list does not contain the ignored function.
	unimplemented!();
	//ASSERT: The parallel list's content is unchanged.
	unimplemented!();

	//ASSERT: Ignoring the function again succeeds.
	unimplemented!();
	//ASSERT: All lists are unchanged.
	unimplemented!();
}

fn test_ignore_parallel_function() {
	let test_lists = setup_test_lists();

	//Test ignoring a parallel function...
	//ASSERT: Ignoring a parallel function succeeds.
	unimplemented!();
	//ASSERT: The ignore list contains the ignored function.
	unimplemented!();
	//ASSERT: The ignore list has only grown by 1.
	unimplemented!();
	//ASSERT: The ignore list's other contents are the same.
	unimplemented!();
	//ASSERT: The parallel list does not contain the ignored function.
	unimplemented!();
	//ASSERT: The parallel list has only shrunk by 1.
	unimplemented!();
	//ASSERT: The parallel list's other contents are the same.
	unimplemented!();
	//ASSERT: The main list does not contain the ignored function.
	unimplemented!();
	//ASSERT: The main list's content is unchanged.
	unimplemented!();

	//ASSERT: Ignoring the function again succeeds.
	unimplemented!();
	//ASSERT: All lists are unchanged.
	unimplemented!();
}

fn test_adding_ignored_function() {
	let test_lists = setup_test_lists();

	//Test adding ignored functions to other lists...
	//ASSERT: Adding a parallel function already in
	//ignored_tests() fails with Err.
	unimplemented!();
	//ASSERT: All of the test lists are unchanged.
	unimplemented!();

	//ASSERT: Adding a main function already in
	//ignored_tests() fails with Err.
	unimplemented!();
	//ASSERT: All of the test lists are unchanged.
	unimplemented!();
}

fn test_ignore_invalid_input() {
	let test_lists = setup_test_lists();

	//Test invalid input...
	//ASSERT: Ignoring a non-function returns Err.
	//(check specific error?)
	unimplemented!();
	//ASSERT: All of the test lists are unchanged.
	unimplemented!();
}

/**
 * Returns all tests in this module.
 */
pub fn all_tests() -> Vec<UnitTest> {
	vec![unit_test!(test_ignore_new_function), unit_test!(test_ignore_main_function), unit_test!(test_ignore_parallel_function), unit_test!(test_adding_ignored_function), unit_test!(test_ignore_invalid_input)]
}