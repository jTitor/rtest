/*!
 * Defines the root of the unit tests for this crate.
*/
//Anything marked "unreachable!()"
//is intentional; it's supposed to indicate
//that the section needs expanding
#![allow(unreachable_code)]

extern crate rtest;

mod discovery;
mod frontend;
mod test_run;

/**
 * Returns all tests in the crate.
 */
fn all_tests() -> Vec<fn()> {
	discovery::all_tests().append(frontend::all_tests().append(test_run::all_tests()))
}