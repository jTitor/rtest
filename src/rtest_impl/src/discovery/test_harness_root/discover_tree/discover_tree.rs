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

	pub fn discover(&self, file: &mut syn::File) -> Result<(), Error> {
		//Iterate over the file's items:
		//For each mod, do a visit.
		unimplemented!();

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
				//let ignore_functions: Vec<
				let parallel_reexports: Vec<String> = vec![];
				let main_reexports: Vec<String> = vec![];

				//Iterate over all the module items:
				let _ = item_vec
					.iter()
					.map(|item| {
						match item {
							//	If they're an ItemFn:
							syn::Item::Fn(fn_item) => {
								//		If they're an important function:
								//			If they're an ignore function:
								//				Add the function name to an ignore list.
								//			Else, they're a test function by implication:
								//				If they're a test-main function:
								//					Add the function name to a test-main list.
								//				Else:
								//					Add the function name to a test list.
							}
							_ => {
								//Isn't a function,
								//add debug text here
							}
						}
					})
					.collect::<Vec<_>>();

				//Reexport test functions. All will be in the format
				//"pub fn test_fn_[function name]() { [function name](); }"
				//Export parallel functions...
				//Export main functions...
				
				//Generate TestEntry lists:
				//For the parallel functions...
				//For the main functions...
				//For the ignored functions...
				//	TODO: will need an alternate IgnoredTestEntry type
				//	for this, since there's no point reexporting ignored functions

				unimplemented!();
			}
			_ => {
				//Isn't a function,
				//add debug text here
			}
		}

		//Next, reexport the list of test functions as
		//pub fn __test_mod_leaf_functions() -> Vec<fn()>.
		unimplemented!();
	}
}

impl VisitMut for DiscoverTree {
	fn visit_item_mod_mut(&mut self, i: &mut syn::ItemMod) {
		//Track the module's name.
		unimplemented!();

		//Reexport all our test functions in this module.
		self.reexport_test_functions(i);

		//Now continue on the subitems.
		visit_mut::visit_item_mod_mut(self, i);

		//Create a list of the modules,
		//and export their function lists (*::__test_mod_leaf_functions)
		//as
		//pub fn __test_mod_submod_functions() -> Vec<fn()>.
		unimplemented!();

		//Export a final function
		//pub fn __test_mod_functions() -> Vec<fn()>
		//that returns self::__test_mod_submod_functions //appended to self::__test_mod_leaf_functions.
		unimplemented!();
	}

	fn visit_item_fn_mut(&mut self, i: &mut syn::ItemFn) {
		//Is this function important?
		if syn_ops::fn_is_important(i) {
			//If so:
			//	Reexport this for testing.
			//That is, !quote a pub fn that just calls this function.
			// Also create a TestEntry for the reexport.
		}

		visit_mut::visit_item_fn_mut(self, i);
	}
}
