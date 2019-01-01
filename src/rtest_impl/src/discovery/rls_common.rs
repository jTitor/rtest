/*!
 * Defines common functions used to interact with RLS.
*/
use std::fmt;

/**
 * Posts the given displayable item
 * as an RLS error.
 */
pub fn post_rls_error<T: fmt::Display>(text: &T) {
	//Report to RLS...
	unimplemented!();
}

/**
 * Posts the given displayable item
 * as an RLS warning.
 */
pub fn post_rls_warning<T: fmt::Display>(text: &T) {
	//Report to RLS...
	unimplemented!();
}