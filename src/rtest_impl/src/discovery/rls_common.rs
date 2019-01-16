/*!
 * Defines common functions used to interact with RLS.
*/
use std::fmt;

/**
 * Posts the given displayable item
 * as an RLS error.
 */
pub fn post_rls_error<T: fmt::Display>(text: &T) {
	//...We can't actually print an *RLS* error
	//as there's no real API for dynamic stringed compiler errors.
	//Print to stderr instead.
	eprintln!("[error] {}", text);
}

/**
 * Posts the given displayable item
 * as an RLS warning.
 */
pub fn post_rls_warning<T: fmt::Display>(text: &T) {
	//...We can't actually print an *RLS* error
	//as there's no real API for dynamic stringed compiler errors.
	//Print to stderr instead.
	eprintln!("[warn] {}", text);
}

/**
 * Posts the given displayable item
 * as an RLS debug warning.
 */
pub fn post_rls_debug_warning<T: fmt::Display>(text: &T) {
	//...We can't actually print an *RLS* error
	//as there's no real API for dynamic stringed compiler errors.
	//Print to stderr instead.
	eprintln!("[debug] {}", text);
}