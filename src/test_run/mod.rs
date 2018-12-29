/*!
 * Defines the ```test_run``` module.
 */
mod runner;
pub use self::runner::TestRunner;

mod failure_detail;
use self::failure_detail::*;