/*!
 * Defines common functions used by other modules
 * in the attributes module.
 */
use proc_macro::TokenStream;

use crate::discovery::TestEntry;
use crate::discovery::rls_common;

/**
 * If true, the given TokenStream represents a function,
 * otherwise it does not represent a function.
 */
pub fn is_function(item: &TokenStream) -> bool {
	unimplemented!();
}

/**
 * Prints a warning to RLS
 * that the given TokenStream is not a function,
 * and so attribute_name will have no effect.
 */
pub fn warn_not_function(item: &TokenStream, attribute_name: &str) {
	let warning_text = format!("TokenStream '{}' does not represent a function, attribute '#[{}]' will have no effect", item, attribute_name);

	rls_common::post_rls_warning(&warning_text);
}

/**
 * Prints a warning to RLS
 * that a TestLists instance couldn't be
 * retrieved, and so attribute_name will have no effect.
 */
pub fn warn_list_instance_failed(attribute_name: &str) {
	let warning_text = format!("Couldn't get test list instance, attribute '#[{}]' will have no effect", attribute_name);

	rls_common::post_rls_warning(&warning_text);
}

/**
 * Generates a TestEntry from the given
 * TokenStream if possible.
 */
pub fn extract_test_entry(item: &TokenStream) -> TestEntry {
	unimplemented!();
}
