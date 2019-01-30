/*!
 * Crate root for the library.
 */

//Anything marked "unimplemented!()"
//is intentional; it's supposed to indicate
//that the section needs expanding
#![allow(unreachable_code)]

//Define our base level modules.

/**
 * Runs the harness entry point generated by init_rtest!().
 * 
 * Call this in your unit test crate's main() function with
 * run_rtest!().
 * This will call std::process::exit(), so there's no point
 * trying to call this more than once.
 */
#[macro_export]
macro_rules! run_rtest {
	() => { run_rtest(); }
}