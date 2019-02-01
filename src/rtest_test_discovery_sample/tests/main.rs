/*!
 * Defines the root module for tests in this crate.
 */
#[macro_use]
extern crate rtest_user_proc_macros;
#[macro_use]
extern crate rtest_user_macros;

//Custom attributes can't be applied to modules,
//so... use a macro instead I guess?
#[test_mod]
mod private_module;
#[test_mod]
pub mod pub_module;

init_rtest!();

/**
 * Entry point for the test harness.
 */
fn main() {
	//Run the unit tests; return 1 if any fail,
	//0 if all pass
	run_rtest!();
}