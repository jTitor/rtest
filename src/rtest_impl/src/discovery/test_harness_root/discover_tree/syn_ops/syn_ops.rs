/*!
 * Defines internal operations used by DiscoverTree
 * that use the syn library.
 */
use syn;
//Rust 2018 has a bug with path names;
//use :: prefix to temporarily fix
use ::proc_macro;

use super::{fn_properties, names};
use crate::discovery::TestEntry;

/**
 * TODO
 */
pub fn generate_test_entry_fn_list(
	export_list_name: String,
	fn_name_list: &Vec<String>,
) -> proc_macro::TokenStream {
	let mut test_entry_list: Vec<TestEntry> = Vec::with_capacity(fn_name_list.len());
	let (entry_list_name, name_list_name, fn_list_name) =
		names::list_names(export_list_name.as_str());

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

/**
 * Generates the Vec<TestEntry> functions for the module's
 * parallel, main, and ignored tests, in that order.
 */
pub fn generate_test_entry_fns_all(
	parallel_test_fns: &Vec<String>,
	main_test_fns: &Vec<String>,
	ignored_test_fns: &Vec<String>,
) -> (
	proc_macro::TokenStream,
	proc_macro::TokenStream,
	proc_macro::TokenStream,
) {
	(
		generate_test_entry_fn_list(names::parallel_lists_name(), parallel_test_fns),
		generate_test_entry_fn_list(names::main_lists_name(), main_test_fns),
		generate_test_entry_fn_list(names::ignored_lists_name(), ignored_test_fns),
	)
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
	let (parallel_list_name, main_list_name, ignore_list_name) = names::test_lists_vec_fn_names();
	let fn_name = names::test_lists_leaves_name();

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
	let fn_name = names::test_lists_nodes_name();
	let submodule_fn_name = names::test_lists_name();
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
	let fn_name = names::test_lists_name();
	let leaves_fn_name = names::test_lists_leaves_name();
	let nodes_fn_name = names::test_lists_nodes_name();

	let expanded = quote! {
		pub fn #fn_name() -> TestLists {
			let mut result = #leaves_fn_name();

			result.append(#nodes_fn_name());

			result
		}
	};

	expanded.into()
}

/**
 * Appends the given TokenStream to the given module's
 * contents Vec.
 *
 * The token stream is assumed to be a list of fn definitions.
 */
pub fn append_fn_tokens_to_mod(module: &mut syn::ItemMod, tokens: proc_macro::TokenStream) {
	unimplemented!();
}

/**
 * Given a list of syn::Items, retrieves all
 * syn::ItemFn instances within the list that
 * are either marked with
 * #[test], #[test(main)], or #[ignore],
 * returning a tuple containing the names of the marked
 * functions in three lists.
 *
 * Returns the function names in the following order:
 *   * 0: parallel tests
 *   * 1: main-thread tests
 *   * 2: ignored tests
 */
pub fn collect_marked_fns(item_vec: &Vec<syn::Item>) -> (Vec<String>, Vec<String>, Vec<String>) {
	let mut ignore_functions: Vec<String> = vec![];
	let mut parallel_functions: Vec<String> = vec![];
	let mut main_functions: Vec<String> = vec![];

	//Filter the module items:
	let _ = item_vec
		.iter()
		.map(|item| {
			match item {
				//	If they're an ItemFn:
				syn::Item::Fn(fn_item) => {
					//If they're an important function:
					if fn_properties::fn_is_important(&fn_item) {
						//If they're an ignore function:
						if fn_properties::fn_has_ignore_attrib(&fn_item) {
							//Add the function name to an ignore list.
							ignore_functions.push(fn_item.ident.to_string());
						} else {
							//Else, they're a test function by implication.

							//If they're a test-main function:
							if fn_properties::fn_has_test_main_attrib(&fn_item) {
								//Add the function name to a main-thread test list.
								main_functions.push(fn_item.ident.to_string());
							} else {
								//Else:
								//Add the function name to a test list.
								parallel_functions.push(fn_item.ident.to_string());
							}
						}
					}
				}
				_ => {
					//Isn't a function,
					//add debug text here
				}
			}
		})
		.collect::<Vec<_>>();

	(parallel_functions, main_functions, ignore_functions)
}
