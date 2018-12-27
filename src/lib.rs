//Crate root for the library.

//Anything marked "unreachable!()"
//is intentional; it's supposed to indicate
//that the section needs expanding
#![allow(unreachable_code)]

extern crate proc_macro;
extern crate failure;

//Define our base level modules.
mod discovery;
mod frontend;
mod test_run;

//Define custom attributes; whoever decided these have to be in the crate root were total assholes
use proc_macro::TokenStream;

/**
 * Marks a function as a test function.
 */
#[proc_macro_attribute]
pub fn test(attr: TokenStream, item: TokenStream) -> TokenStream {
	//Is item a function?
	//If so:
	//	Does attr contain "main"?
	//	If so:
	//		Add item to the main test list.
	//	Else:
	//		Add item to the parallel test list.
	//Else:
	//	Warn that the item isn't a function and that this tag has no effect.

	//Return the input elements as-is.
	item
}

/**
 * Marks a function as a function that must be ignored during testing.
 * It is considered an error to test a function marked as #[ignore].
 */
#[proc_macro_attribute]
pub fn ignore(_attr: TokenStream, item: TokenStream) -> TokenStream {
	//Is item a function?
	//If so:
	//	Remove item from the parallel/main test lists if it is within either list.
	//	Add item to the ignore list.
	//Else:
	//	Warn that the item isn't a function and that this tag has no effect.

	//Return the input elements as-is.
	item
}