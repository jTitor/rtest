/*!
 * TODO
 */
use rtest_impl::frontend::TerminalView;

use crate::UnitTest;

fn test_new_terminal_view() {
	//ASSERT: TerminalView::new()
	//returns a new TerminalView instance
	let _ = TerminalView::new();
}

fn test_log_terminal_view() {
	//Visibility test: TerminalView
	//instance prints the given message
	//on ::log() call
	println!("Running TerminalView::log(); you should see a message below.");
	TerminalView::new().log("Test message!");
}

/**
 * Returns all tests in this module.
 */
pub fn all_tests() -> Vec<UnitTest> {
	vec![
		unit_test!(test_new_terminal_view),
		unit_test!(test_log_terminal_view),
	]
}
