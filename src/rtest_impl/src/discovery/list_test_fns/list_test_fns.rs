/**
 * Defines the DiscoverTree struct.
 */
use failure::Error;
use syn;
use syn::visit;

use std::fs::File;
use std::io::Read;

use crate::discovery;

/**
 * Stores the name of a test function
 * and the name of the function it's representing.
 * 
 * This is used instead of a discovery::TestEntry
 * because the syn traversal can't provide function pointers
 * to the content being traversed; instead we have
 * to use the identifiers provided in a quote!{} to
 * convert them to function pointers at macro time.
 */
#[derive(Default, Debug, Clone)]
pub struct TestFnNamePair {
	pub test_name: String,
	pub fn_name: String
}

/**
 * Stores lists of TestFnNamePairs for
 * all of the test/ignore types used by
 * discovery::TestLists.
 */
#[derive(Default, Debug, Clone)]
pub struct TestFnList {
	//The names of each function we've found
	//that should be run/ignored by the test runner.
	pub parallel_fn_names: Vec<TestFnNamePair>,
	//TODO
	pub main_fn_names: Vec<TestFnNamePair>,
	//TODO
	pub ignore_fn_names: Vec<TestFnNamePair>,
}

impl TestFnList {
	/**
	 * TODO_DESC
	 */
	pub fn clear(&mut self) {
		self.parallel_fn_names.clear();
		self.main_fn_names.clear();
		self.ignore_fn_names.clear();
	}
}

/**
 * TODO_DESC
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
	test_fns: TestFnList,
}

impl TestFnFinder {
	//Private functions.
	/**
	 * TODO_DESC
	 */
	fn push_mod_name(&mut self, name: String) {
		self.mod_names.push(name);
	}

	/**
	 * TODO_DESC
	 */
	fn pop_mod_name(&mut self) {
		self.mod_names.pop();
	}

	/**
	 * TODO_DESC
	 */
	fn mod_name(&self) -> String {
		//TODO: Might need to start with
		//"crate::"; verify in manual testing.
		let mut result = "".to_string();

		for name in &self.mod_names {
			result = format!("{}{}::", result, name);
		}

		result
	}

	/**
	 * TODO_DESC
	 */
	fn add_test_fn(&mut self, fn_name: &str) {
		let full_fn_name = format!("{}_rtest_test_{}", self.mod_name(), fn_name);

		self.test_fns.parallel_fn_names.push(TestFnNamePair {
			test_name: fn_name.to_string(),
			fn_name: full_fn_name
		});
	}

	/**
	 * TODO_DESC
	 */
	fn add_main_test_fn(&mut self, fn_name: &str) {
		let full_fn_name = format!("{}_rtest_test_{}", self.mod_name(), fn_name);

		self.test_fns.main_fn_names.push(TestFnNamePair {
			test_name: fn_name.to_string(),
			fn_name: full_fn_name
		});
	}

	/**
	 * TODO_DESC
	 */
	fn add_ignore_fn(&mut self, fn_name: &str) {
		let full_fn_name = format!("{}_rtest_ignore_{}", self.mod_name(), fn_name);

		self.test_fns.ignore_fn_names.push(TestFnNamePair {
			test_name: fn_name.to_string(),
			fn_name: full_fn_name
		});
	}

	/**
	 * TODO_DESC
	 */
	fn clear_all(&mut self) {
		self.mod_names.clear();
		self.test_fns.clear();
	}

	/**
	 * TODO_DESC
	 */
	fn add_fn<'a>(&mut self, i: &'a syn::ItemFn) {
		//Check attribs:
		for attribute in &i.attrs {
			match attribute.interpret_meta() {
				//Is this a non-parenthesized attribute
				//(#[test], #[ignore])?
				Some(syn::Meta::Word(attrib_name)) => match attrib_name.to_string().to_lowercase().as_str() {
					//TODO - make literals constants
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
				},
				//Else, is this #[test(main)]?
				Some(syn::Meta::List(attrib_list)) => match attrib_list.ident.to_string().to_lowercase().as_str() {
					"test" => {
						//We know it's a #[test] attrib,
						//now see if it contains "main".
						//"main" will also be a Meta::Word
						//with Meta::Word.name().to_string() == "main".
						let contains_main = attrib_list.nested.iter()
						.filter(|x| match x {
							syn::NestedMeta::Meta(syn::Meta::Word(nested_word)) => nested_word.to_string().to_lowercase().as_str() == "main",
							_ => false
						})
						.collect::<Vec<_>>()
						.count() > 0;
						//Note - any other nested
						//attributes besides "main"
						//are ignored

						//If this is #[test(main)],
						//add it to the main-thread test list.
						if contains_main {
							self.add_main_test_fn(attrib_list.ident.to_string().as_str());
						}
						else {
							//Otherwise, add this as a standard test.
							self.add_test_fn(attrib_list.ident.to_string().as_str());
						}
					}
					_ => {}
				},
				_ => {}
			}
		}
	}

	/**
	 * Performs the actual work of find_test_fns().
	 * Since the internal calls can return results,
	 * we can't use the try operator directly in
	 * find_test_fns without undesired early return.
	 */
	fn do_find_test_fns(&mut self, file_path: &str) -> Result<TestFnList, Error> {
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
		Ok(self.test_fns.clone())
	}

	//Public functions.
	/**
	 * Creates a new TestFnFinder instance.
	 */
	pub fn new() -> Self {
		Default::default()
	}

	/**
	 * TODO_DESC
	 */
	pub fn find_test_fns(&mut self, file_path: &str) -> Result<TestFnList, Error> {
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

		//The traversal will add any
		//attrib-marked functions
		visit::visit_item_mod(self, i);

		//We're done with this module,
		//so remove it from the name stack
		self.pop_mod_name();
	}

	fn visit_item_fn(&mut self, i: &'a syn::ItemFn) {
		//Check that this function is testable:
		//	* Takes 0 parameters
		//	* Returns nothing
		//If not, skip it
		let fn_has_no_params = i.decl.inputs.is_empty();
		let fn_has_no_output = match i.decl.output {
			syn::ReturnType::Default => true,
			_ => false
		};

		if fn_has_no_params && fn_has_no_output {
			self.add_fn(i);
		}

		//Call out to super?
		visit::visit_item_fn(self, i);
	}
}
