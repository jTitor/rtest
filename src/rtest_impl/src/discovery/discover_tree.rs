/**
 * Defines the DiscoverTree struct.
 */

const TEST_LISTS_FUNCTION_NAME: &'static str = "__test_lists";

/**
 * Runs attribute calls over
 * a given AST tree, exposing the
 * resulting TestLists instance as a
 * __test_lists() function in the current scope.
 */
pub struct DiscoverTree {}

impl DiscoverTree {
	pub fn discover() {
		//First, run over the tree given.
		//Use the syn::Visit trait to
		//track functions and methods via:
		//	* syn::visit::visit_item_fn
		//	* syn::visit::visit_impl_item_method
		//
		//In both visits, check that the item's attrs
		//contains either test or ignore,
		//and run the respective attribute function.
		//If the item has both, ignore is first priority.
		
		//Once tree traversal is done, get the TestLists.
		//Generate a __test_lists() function that
		//returns the given TestList.

		//If parse failed, panic; there's no point trying
		//to test if we don't have a valid test list.
	}
}