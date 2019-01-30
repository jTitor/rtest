/*!
 * Crate root for the library.
 */

//Anything marked "unimplemented!()"
//is intentional; it's supposed to indicate
//that the section needs expanding
#![allow(unreachable_code)]

extern crate proc_macro;
use proc_macro::TokenStream;

extern crate rtest_impl;
use rtest_impl::macro_impls;

//Define our base level modules.

//Define custom attributes; whoever decided these have to be
//in the crate root were - actually they're not so bad now
//that these are in a separate crate

/**
 * Marks a function as a test function.
 * Can be used as #[test(main)], in which case
 * the function's test will run on the main thread.
 */
#[proc_macro_attribute]
pub fn test(attrib: TokenStream, item: TokenStream) -> TokenStream {
	//Codegen here...
	let expanded = macro_impls::expose_test(attrib, item);

	expanded
}

/**
 * Marks a function to be ignored by the test system.
 * When testing is run, the function will be acknowledged,
 * but will not be executed by the test system.
 */
#[proc_macro_attribute]
pub fn ignore(_attrib: TokenStream, item: TokenStream) -> TokenStream {
	//Codegen here...
	let expanded = macro_impls::expose_ignore(item);

	expanded
}

/**
 * Exposes a module as a test module, reexporting
 * it as a public module if it wasn't already public.
 */
#[proc_macro_attribute]
pub fn test_mod(_attrib: TokenStream, item: TokenStream) -> TokenStream {
	//Codegen here...
	let expanded = macro_impls::expose_test_mod(item);

	expanded
}

/**
 * Initializes rtest for the crate this macro is used in,
 * generating the unit tests and harness entry point.
 * 
 * Apply this macro via init_rtest!() in the
 * root module of your unit test crate.
 * Only apply this macro once.
 */
#[proc_macro]
pub fn init_rtest(_item: TokenStream) -> TokenStream {
	//Apply the codegen...
	let expanded = macro_impls::generate_harness_entry(file!());

	//And return the augmented code	
	expanded
}