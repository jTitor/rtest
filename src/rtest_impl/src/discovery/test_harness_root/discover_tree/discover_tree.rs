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
	//Private functions.
	/**
	 * TODO
	 */
	fn populate_node_and_traverse<NodeT, GetContentFnT, TraverseFnT>(
		&mut self,
		node: &mut NodeT,
		get_content_block: GetContentFnT,
		traverse_block: TraverseFnT,
	) where
		GetContentFnT: Fn(&mut NodeT) -> Option<&mut Vec<syn::Item>>,
		TraverseFnT: Fn(&mut DiscoverTree, &mut NodeT),
	{
		if let Some(content) = get_content_block(node) {
			//Reexport all our test functions in this module.
			self.reexport_test_functions(content);
		}

		//Now traverse the subitems.
		// visit_mut::visit_item_mod_mut(self, i);
		traverse_block(self, node);

		//Get all the submodules...
		let submodule_names = match get_content_block(node) {
			Some(content) => content
				.iter()
				.filter_map(|x| match x {
					syn::Item::Mod(inner) => Some(inner.ident.to_string()),
					_ => None,
				})
				.collect(),
			_ => vec![],
		};

		if let Some(content) = get_content_block(node) {
			//...and export their function lists (*::__test_mod_leaf_functions)
			//as
			//pub fn __test_mod_submod_functions() -> Vec<fn()>.
			syn_ops::append_fn_tokens_to_content_list(
				content,
				syn_ops::generate_test_lists_nodes_fn(&submodule_names),
			);

			//Export a final function
			//pub fn __test_mod_functions() -> Vec<fn()>
			//that returns self::__test_mod_submod_functions //appended to self::__test_mod_leaf_functions.
			syn_ops::append_fn_tokens_to_content_list(content, syn_ops::generate_test_lists_fn());
		}
	}

	/**
	 * TODO
	 */
	fn reexport_test_functions(&mut self, content: &mut Vec<syn::Item>) {
		let (parallel_test_names, main_test_names, ignored_test_names) =
			syn_ops::collect_marked_fns(content);

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
			syn_ops::append_fn_tokens_to_content_list(content, list);
		}

		//Next, reexport the list of test functions as
		//pub fn __test_mod_leaf_functions() -> Vec<fn()>.
		syn_ops::append_fn_tokens_to_content_list(
			content,
			syn_ops::generate_test_lists_leaves_fn(),
		);
	}

	//Public functions.
	pub fn new() -> Self {
		Self {}
	}

	pub fn discover(&mut self, file: &mut syn::File) -> Result<(), Error> {
		//Traverse this file and its modules.
		self.populate_node_and_traverse(
			file,
			|file| { Some(&mut file.items) },
			|v, file| {
				visit_mut::visit_file_mut(v, file);
			},
		);

		//Also generate a function __run_test_harness() that'll
		//make a Runner, then
		//call Runner::run(self::__test_mod_functions()).
		unimplemented!();

		Ok(())
	}
}

impl VisitMut for DiscoverTree {
	fn visit_item_mod_mut(&mut self, i: &mut syn::ItemMod) {
		self.populate_node_and_traverse(
			i,
			|i| match i.content {
				Some((_, ref mut content_inner)) => Some(content_inner),
				_ => None,
			},
			|v, i| {
				visit_mut::visit_item_mod_mut(v, i);
			},
		);
	}
}
