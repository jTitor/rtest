/*!
 * Defines the attributes module.
 */
mod common;

mod ignore;

mod test;

/**
 * Returns all tests in this module.
 */
pub fn all_tests() -> Vec<fn()> {
	common::all_tests().append(ignore::all_tests().append(test::all_tests()))
}