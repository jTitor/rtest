/*!
 * Defines the ```discovery``` module.
 */
mod rls_common;

mod test_entry;
pub use self::test_entry::*;

mod test_lists;
pub use self::test_lists::*;

mod static_test_list;
pub use self::static_test_list::*;

mod attributes;
pub use self::attributes::*;
