/*!
 * Crate root for the library.
 */

//Anything marked "unreachable!()"
//is intentional; it's supposed to indicate
//that the section needs expanding
#![allow(unreachable_code)]

extern crate failure;
extern crate proc_macro;
extern crate rayon;
extern crate quote;
extern crate syn;

//Define our base level modules.
pub mod discovery;
pub use self::discovery::{do_attribute_ignore, do_attribute_test};

pub mod frontend;
pub mod test_run;

pub use self::test_run::TestRunner;
