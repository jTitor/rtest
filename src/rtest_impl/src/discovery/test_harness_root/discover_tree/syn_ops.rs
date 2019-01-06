/*!
 * Defines internal operations used by DiscoverTree
 * that use the syn library.
 */
use failure::Error;
use syn;
//Rust 2018 has a bug with path names;
//use :: prefix to temporarily fix
use ::proc_macro;
use ::quote;

use crate::discovery::TestEntry;

/**
 * Returns true if a function should be
 * visible to the testing system, either
 * for testing or for ignoring.
 * A function's important if all of the following apply:
 * 	* Has either a ![test] OR ![ignore] attribute
 * 	* AND has no parameters
 * 	* AND returns nothing
 */
pub fn fn_is_important(fn_item: &syn::ItemFn) -> bool {
	unimplemented!();
}

/**
 * Returns true if the given function has
 * a #[test] attribute.
 */
pub fn fn_has_test_attrib(fn_item: &syn::ItemFn) -> bool {
	unimplemented!();
}

/**
 * Returns true if the given function has
 * a #[test(main)] attribute.
 *
 * If true, implies fn_has_test_attrib() is true.
 */
pub fn fn_has_test_main_attrib(fn_item: &syn::ItemFn) -> bool {
	unimplemented!();
}

/**
 * Returns true if the given function has
 * a #[ignore] attribute.
 */
pub fn fn_has_ignore_attrib(fn_item: &syn::ItemFn) -> bool {
	unimplemented!();
}

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

pub fn generate_test_entry_fn_list(
	export_list_name: String,
	fn_name_list: &Vec<String>,
) -> proc_macro::TokenStream {
	let mut test_entry_list: Vec<TestEntry> = Vec::with_capacity(fn_name_list.len());
	let (entry_list_name, name_list_name, fn_list_name) = list_names(export_list_name.as_str());

	//Now generate a *fully expanded*
	//list of TestEntries for the given
	//function name list...
	let expanded = quote! {
		pub fn #name_list_name() -> Vec<String> {
			vec![#(#fn_name_list),*]
		}

		pub fn #fn_list_name() -> Vec<fn()> {
			vec![#(test_fn_#fn_name_list),*]
		}

		pub fn #entry_list_name() -> Vec<TestEntry> {
			#name_list_name().iter()
			.zip(#fn_list_name().iter())
			.map(|(fn_name, fn_pointer)| {
				TestEntry {
					name: fn_name,
					test: fn_pointer,
				}
			})
			.collect::<Vec<TestEntry>>()
		}
	};

	expanded.into()
}

//Name constants used by the generate_test_lists_*
//functions to follow.
pub fn test_lists_leaves_name() -> String {
	"__test_lists_leaves".into()
}

pub fn test_lists_nodes_name() -> String {
	"__test_lists_nodes".into()
}

pub fn test_lists_name() -> String {
	"__test_lists".into()
}

pub fn parallel_lists_name() -> String {
	"parallel_tests".into()
}

pub fn main_lists_name() -> String {
	"main_tests".into()
}

pub fn ignored_lists_name() -> String {
	"ignored_tests".into()
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

/**
 * Generates the Vec<TestEntry> functions for the module's
 * parallel, main, and ignored tests, in that order.
 */
pub fn generate_test_entry_fns_all(parallel_test_fns: &Vec<String>, main_test_fns: &Vec<String>, ignored_test_fns: &Vec<String>) -> (proc_macro::TokenStream, proc_macro::TokenStream, proc_macro::TokenStream) {
	(generate_test_entry_fn_list(parallel_lists_name(), parallel_test_fns), generate_test_entry_fn_list(main_lists_name(), main_test_fns), generate_test_entry_fn_list(ignored_lists_name(), ignored_test_fns))
}

/**
 * Generates a TokenStream representing
 * a function that returns a TestLists containing
 * all of the marked functions as TestEntry instances.
 */
pub fn generate_test_lists_leaves_fn() -> proc_macro::TokenStream {
	//Uses the Vec<TestEntry> fns, so
	//you should use the entry_list_name() fn to
	//retrieve the vec fns for each list.
	//
	//parallel_list_name should be __entries_for_parallel_list,
	//etc.
	let (parallel_list_name, main_list_name, ignore_list_name) = test_lists_vec_fn_names();

	let fn_name = test_lists_leaves_name();

	let expanded = quote! {
		pub fn #fn_name() -> TestLists {
			if let Ok(result_list) = TestLists::from_test_entries(#parallel_list_name(), #main_list_name(), #ignore_list_name()) {
				return result_list;
			}

			return TestLists::new();
		}
	};

	expanded.into()
}

/**
 * Generates a TokenStream representing
 * a function that returns a Vec<TestLists> of
 * the TestLists functions for each submodule in the given list.
 */
pub fn generate_test_lists_nodes_fn(submodule_names: &Vec<String>) -> proc_macro::TokenStream {
	let fn_name = test_lists_nodes_name();
	let submodule_fn_name = test_lists_name();
	let appended_submodule_names = submodule_names
		.iter()
		.map(|name| format!("{}::{}", name, submodule_fn_name).to_string())
		.collect::<Vec<String>>();

	let expanded = quote! {
		pub fn #fn_name() -> Vec<TestLists> {
			vec![#(#appended_submodule_names()),*]
		}
	};

	expanded.into()
}

/**
 * Generates a TokenStream representing
 * a function that returns a TestLists containing
 * all of the TestEntry instances in this module and its submodules.
 */
pub fn generate_test_lists_fn() -> proc_macro::TokenStream {
	let fn_name = test_lists_name();
	let leaves_fn_name = test_lists_leaves_name();
	let nodes_fn_name = test_lists_nodes_name();

	let expanded = quote! {
		pub fn #fn_name() -> TestLists {
			let mut result = #leaves_fn_name();

			result.append(#nodes_fn_name());

			result
		}
	};

	expanded.into()
}
