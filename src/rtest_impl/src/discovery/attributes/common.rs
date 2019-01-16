/*!
 * Defines common functions used by other modules
 * in the attributes module.
 */
use proc_macro::TokenStream;

use crate::discovery::TestEntry;
use crate::discovery::rls_common;

/**
 * If true, the given TokenStream contains
 * a "main" element in its attributes.
 */
pub fn attr_contains_main(attr: &TokenStream) -> bool {
	unimplemented!();
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

pub fn debug_warn_test_add_failed(test_name: &str, error_message: &str) {
	let warning_text = format!(
						"Warning: couldn't add test '{}': {}",
						test_name, error_message
					);

	rls_common::post_rls_debug_warning(&warning_text);
}