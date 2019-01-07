/**
 * Defines the DiscoverTree struct.
 */
use failure::Error;
use syn;
use syn::visit_mut;
use syn::visit_mut::VisitMut;

use super::syn_ops;

const TEST_LISTS_FUNCTION_NAME: &'static str = "__test_lists";

/**
 * Runs attribute calls over
 * a given AST tree, exposing the
 * resulting TestLists instance as a
 * __test_lists() function in the current scope.
 */
pub struct DiscoverTree {}

impl DiscoverTree {
	pub fn new() -> Self {
		Self {}
	}

	pub fn discover(&mut self, file: &mut syn::File) -> Result<(), Error> {
		//Get the modules in this file.
		//For each mod, do a visit.
		let _ = file
			.items
			.iter_mut()
			.map(|x| {
				if let syn::Item::Mod(inner) = x {
					self.visit_item_mod_mut(inner);
				}
			})
			.collect::<Vec<_>>();

		//Now those modules should each have a
		//__test_mod_functions() listing
		//their test functions.
		//Generate a __test_mod_functions() in this file
		//that concatenates the results of each
		//module's __test_mod_functions() function.
		unimplemented!();

		//Also generate a function __run_test_harness() that'll
		//make a Runner, then
		//call Runner::run(self::__test_mod_functions()).
		unimplemented!();

		Ok(())
	}

	fn reexport_test_functions(&mut self, i: &mut syn::ItemMod) {
		match &mut i.content {
			Some((_, ref mut item_vec)) => {
				let (parallel_test_names, main_test_names, ignored_test_names) =
					syn_ops::collect_marked_fns(item_vec);

				//Reexport test functions and
				//generate TestEntry lists for
				//all three fn types:
				//	TODO: will need an alternate IgnoredTestEntry type
				//	for this, since there's no point reexporting ignored functions
				let test_entry_lists = syn_ops::generate_test_entry_fns_all(
					&parallel_test_names,
					&main_test_names,
					&ignored_test_names,
				);
				for list in test_entry_lists.into_iter() {
					syn_ops::append_fn_tokens_to_mod(i, list);
				}
			}
			_ => {
				//Isn't a function,
				//add debug text here
			}
		}

		//Next, reexport the list of test functions as
		//pub fn __test_mod_leaf_functions() -> Vec<fn()>.
		syn_ops::append_fn_tokens_to_mod(i, syn_ops::generate_test_lists_leaves_fn());
	}
}

impl VisitMut for DiscoverTree {
	fn visit_item_mod_mut(&mut self, i: &mut syn::ItemMod) {
		//Reexport all our test functions in this module.
		self.reexport_test_functions(i);

		//Now traverse the subitems.
		visit_mut::visit_item_mod_mut(self, i);

		//Get all the submodules...
		let submodule_names = match &i.content {
			Some((_, item_vec)) => item_vec
				.iter()
				.filter_map(|x| match x {
					syn::Item::Mod(inner) => Some(inner.ident.to_string()),
					_ => None,
				})
				.collect(),
			_ => vec![],
		};

		//...and export their function lists (*::__test_mod_leaf_functions)
		//as
		//pub fn __test_mod_submod_functions() -> Vec<fn()>.
		syn_ops::append_fn_tokens_to_mod(
			i,
			syn_ops::generate_test_lists_nodes_fn(&submodule_names),
		);

		//Export a final function
		//pub fn __test_mod_functions() -> Vec<fn()>
		//that returns self::__test_mod_submod_functions //appended to self::__test_mod_leaf_functions.
		syn_ops::append_fn_tokens_to_mod(i, syn_ops::generate_test_lists_fn());
	}
}
