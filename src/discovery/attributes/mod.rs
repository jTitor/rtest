/*!
 * Defines the attributes module.
 */
mod common;

mod ignore;
pub use self::ignore::do_attribute_ignore;

mod test;
pub use self::test::do_attribute_test;