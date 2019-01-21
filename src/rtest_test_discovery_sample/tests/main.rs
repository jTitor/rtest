/*!
 * Defines the root module for tests in this crate.
 */
#[macro_use]
extern crate rtest_user_macros;

#[init_rtest]
mod tests;

/**
 * Entry point for the test harness.
 */
fn main() {
	//Run the unit tests; return 1 if any fail,
	//0 if all pass
	run_rtest!();
}