/*!
 * Example functions used by super::ignore and super::test.
 */
use proc_macro::TokenStream;

/**
 * Returns a TokenStream for a
 * dummy function named "main_fn_1",
 * intended for adding to a TestLists instance's
 * main-thread list.
 */
pub fn main_fn_1() -> TokenStream {
	let expanded = quote! {
		fn main_fn_1() {}
	};

	expanded.into()
}

/**
 * Returns a TokenStream for a
 * dummy function named "main_fn_2",
 * intended for adding to a TestLists instance's
 * main-thread list.
 */
pub fn main_fn_2() -> TokenStream {
	let expanded = quote! {
		fn main_fn_2() {}
	};

	expanded.into()
}

/**
 * Returns a TokenStream for a
 * dummy function named "parallel_fn_1",
 * intended for adding to a TestLists instance's
 * non-main-thread list.
 */
pub fn parallel_fn_1() -> TokenStream {
	let expanded = quote! {
		fn parallel_fn_1() {}
	};

	expanded.into()
}

/**
 * Returns a TokenStream for a
 * dummy function named "parallel_fn_2",
 * intended for adding to a TestLists instance's
 * non-main-thread list.
 */
pub fn parallel_fn_2() -> TokenStream {
	let expanded = quote! {
		fn parallel_fn_2() {}
	};

	expanded.into()
}