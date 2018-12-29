/*!
 * Defines the attributes module.
 */
mod common;

mod ignore;

mod test;

use ::tests::UnitTest;
/**
 * Returns all tests in this module.
 */
pub fn all_tests() -> Vec<UnitTest> {
	let mut result = common::all_tests();

	result.append(&mut ignore::all_tests());
	result.append(&mut test::all_tests());

	result
}