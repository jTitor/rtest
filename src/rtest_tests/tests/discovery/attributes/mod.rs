/*!
 * Defines the attributes module.
 */
mod common;

use crate::UnitTest;
/**
 * Returns all tests in this module.
 */
pub fn all_tests() -> Vec<UnitTest> {
	let result = common::all_tests();

	result
}