/*!
 * Defines the frontend module.
 */
mod event_log;

mod frontend;

mod static_frontend;

mod terminal_view;



/**
 * Returns all tests in this module.
 */
pub fn all_tests() -> Vec<fn()> {
	terminal_view::all_tests().append(event_log::all_tests().append(frontend::all_tests().append(static_frontend::all_tests())))
}