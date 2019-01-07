/*!
 * Defines functions that test if functions
 * have specific properties that make them significant
 * to the syn_ops::* methods.
 */
use syn;

/**
 * Returns true if a function should be
 * visible to the testing system, either
 * for testing or for ignoring.
 * A function's important if all of the following apply:
 * 	* Has either a ![test] OR ![ignore] attribute
 * 	* AND has no parameters
 * 	* AND returns nothing
 */
pub fn fn_is_important(fn_item: &syn::ItemFn) -> bool {
	unimplemented!();
}

/**
 * Returns true if the given function has
 * a #[test] attribute.
 */
pub fn fn_has_test_attrib(fn_item: &syn::ItemFn) -> bool {
	unimplemented!();
}

/**
 * Returns true if the given function has
 * a #[test(main)] attribute.
 *
 * If true, implies fn_has_test_attrib() is true.
 */
pub fn fn_has_test_main_attrib(fn_item: &syn::ItemFn) -> bool {
	unimplemented!();
}

/**
 * Returns true if the given function has
 * a #[ignore] attribute.
 */
pub fn fn_has_ignore_attrib(fn_item: &syn::ItemFn) -> bool {
	unimplemented!();
}