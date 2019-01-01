/*!
 * Defines common functions used by this module.
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
