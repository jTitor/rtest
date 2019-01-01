/*!
 * Defines common functions used by other modules
 * in the attributes module.
 */
use proc_macro::TokenStream;

use crate::discovery::TestEntry;

/**
 * If true, the given TokenStream represents a function,
 * otherwise it does not represent a function.
 */
pub fn is_function(item: &TokenStream) -> bool {
	unimplemented!();
}

pub fn warn_not_function(item: &TokenStream, attribute_name: String) {
	let warning_text = format!("TokenStream '{}' does not represent a function, attribute '#[{}]' will have no effect", item, attribute_name);

	unimplemented!();
}

pub fn warn_list_instance_failed(attribute_name: String) {
	let warning_text = format!("Couldn't get test list instance, attribute '#[{}]' will have no effect", attribute_name);

	unimplemented!();
}

pub fn extract_test_entry(item: &TokenStream) -> TestEntry {
	unimplemented!();
}
