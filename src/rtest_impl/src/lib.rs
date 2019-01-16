/*!
 * Crate root for the library.
 */

//Anything marked "unreachable!()"
//is intentional; it's supposed to indicate
//that the section needs expanding
#![allow(unreachable_code)]
//Syn needs a bunch of recursions
//to generate the test identifier code
#![recursion_limit = "256"]

extern crate failure;
extern crate proc_macro;
extern crate rayon;
extern crate syn;

#[macro_use]
extern crate quote;
#[macro_use]
extern crate rtest_tool_macros;

//Define our base level modules.
pub mod discovery;
pub use self::discovery::do_test_harness_root;

pub mod frontend;
pub mod test_run;

pub use self::test_run::TestRunner;
