/**
 * Defines the DiscoverTree struct.
 */
use failure::Error;
use syn;

const TEST_LISTS_FUNCTION_NAME: &'static str = "__test_lists";

/**
 * Runs attribute calls over
 * a given AST tree, exposing the
 * resulting TestLists instance as a
 * __test_lists() function in the current scope.
 */
pub struct DiscoverTree {}

impl DiscoverTree {
	pub fn new() -> Self { Self {} }

	pub fn discover(&self, file: &mut syn::File) -> Result<(), Error> {
		//First, parse the current file (as parsed_file).
		//This will give a list of modules
		//of type ItemMod in parsed_file.items.
		//For each of those modules, you call
		//Visit::visit_item_mod(module).
		//
		//Your syn::Visit implementation needs to 
		//have some way to uniquely identify each
		//marked function/module; if you can get the
		//full name that would be best,
		//but if such a thing's not available
		//you can dynamically build the path
		//by implementing the visit_item_mod() method too.

		//Use the syn::Visit trait to
		//track functions via:
		//	* Visit::visit_item_fn()
		//In both visits, check that the item's attrs
		//contains either test or ignore,
		//and that the item doesn't take parameters,
		//and run the respective attribute function.
		//If the item has both, ignore is first priority.
		//
		//The TestEntry for the item will be:
		//TestEntry { name: (item's identifier),
		//	test: (item's full path) }
		//Which implies all tests must also be pub;
		//if it's not we'd have to generate a pub instead,
		//and if the module wasn't pub we'd need to create //a pub counterpart module in the module's parent.
		//Instead, just warn that non-pub content gets ignored.
		//Which now raises the question of what's pub...
		//If the module's pub, then its pub fields are pub too.
		//if the module isn't, but there's a pub use in its parent, then it *should* be pub,
		//but it's hard to tell what can and can't be resolved this way.
		//You'd need two cases - one where the pub use
		//path is explicit, and one where it's a wildcard,
		//which you'd then have to pass as info to the 
		//visit_item_mod() method.
		//...so given all this, just require that the module
		//be pub, and that the method be pub.

		/*
		An alternate would be to generate a pub module at each module visited, which contains a list of functions marked by the desired attributes.
		Each pub function visited would add its entry to a list, which would then be quoted into the debug pub module; each module that wasn't a leaf would then quote the leaves as part of their lists. For non-pub functions it'd be similar, except you generate a pub wrapper method that calls
		the non-pub function, then add an entry for that instead.
		This would allow getting non-pub functions;
		methods can never be added under this system, since
		they take a self parameter.
		*/

		/*
		How do we add these debug modules/calls?
		Each node we visit has an Items list,
		so we'd use quote!() to generate the modules/functions,
		then append the results to the input items.

		If we're going in as a #![macro],
		then we know our root is that attribute's item and
		can do all the necessary generated code
		by converting the TokenStream as syn::File.
		(is it okay that we're traversing *everything*
		in this one attrib macro call?)

		We can also know it's only meant to be done once, so if we notice the attrib's called twice
		we can raise an error.
		*/
		
		//Once tree traversal is done, get the TestLists.
		//Generate a __test_lists() function that
		//returns the given TestList.

		//If parse failed, panic; there's no point trying
		//to test if we don't have a valid test list.
		unimplemented!();
	}
}