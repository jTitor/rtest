/*!
 * Defines the functions that perform the implementations
 * of the macros defined in the rtest_user_macros crate.
 */
use failure::Error;
use ::proc_macro;
use proc_macro::TokenStream;
use quote::ToTokens;

use crate::discovery::TestFnFinder;

/**
 * TODO_DESC
 */
fn get_fn_name(fn_stream: &TokenStream) -> Result<String, Error> {
	let cast_item = syn::parse::<syn::ItemFn>(fn_stream.clone());

	match cast_item {
		Err(e) => Err(e.into()),
		Ok(inner) => Ok(inner.ident.to_string()),
	}
}

/**
 * Marks a function as a test function.
 * Can be used as #[test(main)], in which case
 * the function's test will run on the main thread.
 */
pub fn expose_test(attrib: TokenStream, item: TokenStream) -> TokenStream {
	//Get the function name so we can expose the function
	//properly.
	let fn_name = {
		let name_result = get_fn_name(&item);

		if let Err(_) = name_result {
			return item;
		}

		name_result.unwrap()
	};

	//Does attrib.tolower() contain "main"?
	let contains_main = attrib
		.clone()
		.into_iter()
		.filter(|x| match x {
			proc_macro::TokenTree::Literal(inner) => inner.to_string().to_lowercase() == "main",
			_ => false,
		})
		.count()
		> 0;
	//	If so, tokenstream should create a main test that
	//	calls the function.
	let expose_fn: TokenStream = {
		if contains_main {
			let main_expose_fn = quote! {
				pub fn test_#fn_name() {
					#fn_name();
					unimplemented!();
				}
			};

			main_expose_fn.into()
		} else {
			//	Else, tokenstream should create a standard test
			//	that calls the function.
			let standard_expose_fn = quote! {
				pub fn _rtest_test_#fn_name() {
					#fn_name();
					unimplemented!();
				}
			};

			standard_expose_fn.into()
		}
	};

	//Create token stream to augment input with.
	let mut expanded = item.clone();
	expanded.extend(expose_fn);

	expanded
}

/**
 * Marks a function to be ignored by the test system.
 * When testing is run, the function will be acknowledged,
 * but will not be executed by the test system.
 */
pub fn expose_ignore(item: TokenStream) -> TokenStream {
	//Get the function name so we can expose the function
	//properly.
	let fn_name = {
		let name_result = get_fn_name(&item);

		if let Err(_) = name_result {
			return item;
		}

		name_result.unwrap()
	};

	//Codegen here...
	let expose_fn: TokenStream = quote! {
		pub fn _rtest_ignore_#fn_name() {
			#fn_name();
			unimplemented!();
		}
	}
	.into();

	//Create token stream to augment input with.
	let mut expanded = item.clone();
	expanded.extend(expose_fn);

	expanded
}

/**
 * Exposes a module as a test module, reexporting
 * it as a public module if it wasn't already public.
 */
pub fn expose_test_mod(item: TokenStream) -> TokenStream {
	//Get the input item as a ItemMod.
	let cast_item = syn::parse::<syn::ItemMod>(item.clone());

	match cast_item {
		Err(_) => {
			//If conversion fails, return as is.
			item
		}

		Ok(mut inner) => {
			//Otherwise, mark mod as pub.
			inner.vis = syn::Visibility::Public(syn::VisPublic {
				pub_token: Default::default(),
			});

			//Convert back to TokenStream and return that.
			inner.into_token_stream().into()
		}
	}
}

/**
 * Initializes rtest for the crate this macro is used in,
 * generating the unit tests and harness entry point.
 *
 * Apply this macro via init_rtest!() in the
 * root module of your unit test crate.
 * Only apply this macro once.
 */
pub fn generate_harness_entry(file_path: &str) -> TokenStream {
	//Send a syn traverse to the given file.
	//Use the output to define our list of test fn calls.
	let exposed_fn_list = {
		match TestFnFinder::new().find_test_fns(file_path) {
			Ok(inner) => inner,
			_ => Default::default(),
		}
	};

	//We don't have ToTokens for TestLists,
	//so destructure the TestFnList
	//into vecs and restructure inside the quote!{}
	let (test_pairs, test_main_pairs, ignore_pairs) = (
		exposed_fn_list.parallel_fn_names,
		exposed_fn_list.main_fn_names,
		exposed_fn_list.ignore_fn_names,
	);
	let (test_names, test_fns): (Vec<_>, Vec<_>) = test_pairs
		.into_iter()
		.map(|x| (x.test_name, x.fn_name))
		.unzip();
	let (test_main_names, test_main_fns): (Vec<_>, Vec<_>) = test_main_pairs
		.into_iter()
		.map(|x| (x.test_name, x.fn_name))
		.unzip();
	let (ignore_names, ignore_fns): (Vec<_>, Vec<_>) = ignore_pairs
		.into_iter()
		.map(|x| (x.test_name, x.fn_name))
		.unzip();

	//Apply the codegen...
	let expanded = quote! {
		extern crate rtest_impl;
		use rtest_impl::discovery::{TestEntry, TestLists};
		use rtest_impl::Runner;

		pub fn run_rtest() {
			::std::process::exit({//TODO - if this expands to a Vec<String>
			//instead of Vec<fn>, we're going to
			//have to find another way
			let test_names = vec![#(stringify!(#test_names)),*];
			let test_main_names = vec![#(stringify!(#test_main_names)),*];
			let ignore_names = vec![#(stringify!(#ignore_names)),*];
			let test_fns = vec![#(#test_fns),*];
			let test_main_fns = vec![#(#test_main_fns),*];
			let ignore_fns = vec![#(#ignore_fns),*];

			let test_entries = TestEntry::vec_from_vecs(test_names, test_fns);
			let test_main_entries TestEntry::vec_from_vecs(test_main_names, test_main_fns);
			let ignore_entries TestEntry::vec_from_vecs(ignore_names, ignore_fns);

			let test_list = TestLists::from_test_entries(test_entries, test_main_entries, ignore_entries);

			//Actually run the test runner...
			match Runner::new()
			.run(test_list) {
				Ok(_) => 0,
				_ => 1
			}});
		}
	}
	.into();

	//And return the augmented code
	expanded
}
