/*!
 * Defines the frontend module.
 */
mod event_log;

mod frontend;

mod static_frontend;

mod terminal_view;

use ::tests::UnitTest;

/**
 * Returns all tests in this module.
 */
pub fn all_tests() -> Vec<UnitTest> {
	let mut result = terminal_view::all_tests();
	
	result.append(&mut event_log::all_tests());
	result.append(&mut frontend::all_tests());
	result.append(&mut static_frontend::all_tests());

	result
}