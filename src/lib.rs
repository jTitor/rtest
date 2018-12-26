//Crate root for the library.

//Anything marked "unreachable!()"
//is intentional; it's supposed to indicate
//that the section needs expanding
#![allow(unreachable_code)]

#[macro_use]
extern crate failure;

//Define our base level modules.
mod discovery;
mod frontend;
mod test_run;