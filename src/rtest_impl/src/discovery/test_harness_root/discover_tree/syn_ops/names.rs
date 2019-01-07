/*!
 * Defines functions that provide names for items
 * generated by syn_ops::* functions.
 */

/**
 * TODO
 */
pub fn entry_list_name(export_list_name: &str) -> String {
	format!("__entries_for_{}", export_list_name).into()
}

/**
 * Returns the names of the list functions
 * for a given entry list name.
 * Return elements:
 * 	0: TestEntry list (__entries_for_*)
 * 	1: Function name list (__names_for_*)
 * 	2: Function pointer list (__fns_for_*)
 */
pub fn list_names(export_list_name: &str) -> (String, String, String) {
	(
		entry_list_name(export_list_name),
		format!("__names_for_{}", export_list_name).into(),
		format!("__fns_for_{}", export_list_name).into(),
	)
}

/**
 * Returns the names of the functions
 * that return the Vec<TestEntry> instances
 * for the module's parallel, main, and ignored tests,
 * in that order.
 */
pub fn test_lists_vec_fn_names() -> (String, String, String) {
	(
		entry_list_name(parallel_lists_name().as_str()),
		entry_list_name(main_lists_name().as_str()),
		entry_list_name(ignored_lists_name().as_str()),
	)
}

//Name constants used by the generate_test_lists_*
//functions to follow.
pub fn test_lists_leaves_name() -> String {
	"__test_lists_leaves".into()
}

/**
 * TODO
 */
pub fn test_lists_nodes_name() -> String {
	"__test_lists_nodes".into()
}

/**
 * TODO
 */
pub fn test_lists_name() -> String {
	"__test_lists".into()
}

/**
 * TODO
 */
pub fn parallel_lists_name() -> String {
	"parallel_tests".into()
}

/**
 * TODO
 */
pub fn main_lists_name() -> String {
	"main_tests".into()
}

/**
 * TODO
 */
pub fn ignored_lists_name() -> String {
	"ignored_tests".into()
}

/**
 * TODO
 */
pub fn run_test_harness_fn_name() -> String {
	"__run_test_harness".into()
}