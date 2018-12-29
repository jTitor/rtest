/*!
 * Crate root for the library.
 */

//Anything marked "unreachable!()"
//is intentional; it's supposed to indicate
//that the section needs expanding
#![allow(unreachable_code)]

extern crate proc_macro;
extern crate rtest_impl;

//Define our base level modules.

//Define custom attributes; whoever decided these have to be in the crate root were total assholes
use proc_macro::TokenStream;

/**
 * Marks a function as a test function.
 */
#[proc_macro_attribute]
pub fn test(attr: TokenStream, item: TokenStream) -> TokenStream {
	rtest_impl::do_attribute_test(&attr, &item);

	//Return the input elements as-is.
	item
}

/**
 * Marks a function as a function that must be ignored during testing.
 * It is considered an error to test a function marked as #[ignore].
 */
#[proc_macro_attribute]
pub fn ignore(attr: TokenStream, item: TokenStream) -> TokenStream {
	rtest_impl::do_attribute_ignore(&attr, &item);

	//Return the input elements as-is.
	item
}