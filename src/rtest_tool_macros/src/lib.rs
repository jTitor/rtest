/*!
 * Defines the macros exposed by this crate.
 */

//Imports for the unstable implementation of compile_warning!().
// extern crate proc_macro;
//
// use proc_macro::{Span, TokenStream};


/**
 * Creates a compiler warning at the area called.
 * Unfortunately, you can only place this once per scope...
 */
#[macro_export]
macro_rules! compile_warning {
	($expr:expr) => {
		const WARNING: &str = $expr;
	};
}

//Unstable version of compile_warning!().
//Would be nice to use once this/the lint API is stabilized.
// #[proc_macro]
// pub fn compile_warning(input: TokenStream) -> TokenStream {
// 	//Put a warning where we've been called.
// 	Span::call_site()
// 	.warning(input.to_string())
// 	.emit();

// 	//No need to emit anything else
// 	TokenStream::new()
// }
