/**
 * Defines the DiscoverTree struct.
 */
use failure::Error;
use syn;
use syn::visit;

use std::fs::File;
use std::io::Read;

/**
 * TODO
 *
 * Runs attribute calls over
 * a given AST tree, exposing the
 * resulting TestLists instance as a
 * __test_lists() function in the current scope.
 */
#[derive(Debug, Clone, Default)]
pub struct TestFnFinder {
	//The names of the modules we're currently in.
	mod_names: Vec<String>,
	//The names of each function we've found
	//that should be run/ignored by the test runner.
	fn_names: Vec<String>,
}

impl TestFnFinder {
	//Private functions.
	/**
	 * TODO
	 */
	fn push_mod_name(&mut self, name: String) {
		self.mod_names.push(name);
	}

	/**
	 * TODO
	 */
	fn pop_mod_name(&mut self) {
		self.mod_names.pop();
	}

	/**
	 * TODO
	 */
	fn mod_name(&self) -> String {
		//TODO: Might need to start with
		//"crate::"; verify in manual testing.
		let mut result = "".to_string();

		for name in self.mod_names {
			result = format!("{}{}::", result, name);
		}

		result
	}

	/**
	 * TODO
	 */
	fn add_test_fn(&mut self, fn_name: &str) {
		let full_fn_name = format!("{}_rtest_test_{}", self.mod_name(), fn_name);

		self.fn_names.push(full_fn_name);
	}

	/**
	 * TODO
	 */
	fn add_ignore_fn(&mut self, fn_name: &str) {
		let full_fn_name = format!("{}_rtest_ignore_{}", self.mod_name(), fn_name);

		self.fn_names.push(full_fn_name);
	}

	fn clear_all(&mut self) {
		self.mod_names.clear();
		self.fn_names.clear();
	}

	/**
	 * Performs the actual work of find_test_fns().
	 * Since the internal calls can return results,
	 * we can't use the try operator directly in
	 * find_test_fns without undesired early return.
	 */
	fn do_find_test_fns(&mut self, file_path: &str) -> Result<Vec<String>, Error> {
		//Open the file.
		let mut file = File::open(file_path)?;
		let mut content = String::new();
		file.read_to_string(&mut content)?;

		//Parse it to tokens...
		let file_tokens = syn::parse_file(&content)?;
		//Traverse the tokens.
		visit::visit_file(self, &file_tokens);

		//self.fn_names will contain our
		//fully-pathed function names, so return that
		Ok(self.fn_names.clone())
	}

	//Public functions.
	/**
	 * Creates a new TestFnFinder instance.
	 */
	pub fn new() -> Self {
		Default::default()
	}

	/**
	 * TODO
	 */
	pub fn find_test_fns(&mut self, file_path: &str) -> Result<Vec<String>, Error> {
		self.clear_all();

		let result = self.do_find_test_fns(file_path);
		//Cleanup inner state in case there's
		//other public fns implemented
		self.clear_all();

		result
	}
}

impl<'a> visit::Visit<'a> for TestFnFinder {
	fn visit_item_mod(&mut self, i: &'a syn::ItemMod) {
		//Update name state.
		self.push_mod_name(i.ident.to_string());

		visit::visit_item_mod(self, i);

		self.pop_mod_name();
	}

	fn visit_item_fn(&mut self, i: &'a syn::ItemFn) {
		//Check attribs:
		for attribute in i.attrs {
			if let Some(syn::Meta::Word(attrib_name)) = attribute.interpret_meta() {
				match attrib_name.to_string().as_str() {
					"test" => {
						//Does this have #[test]?
						//If so, generate name for test function.
						//Add it to the list.
						self.add_test_fn(i.ident.to_string().as_str());
					}
					"ignore" => {
						//Else, does this have #[ignore]?
						//If so, generate name for ignore function.
						//Add it to the list.
						self.add_ignore_fn(i.ident.to_string().as_str());
					}
					_ => {}
				}
			}
		}

		//Call out to super?
		visit::visit_item_fn(self, i);
	}
}
