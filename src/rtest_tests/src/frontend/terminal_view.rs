/*!
 * TODO
 */
use crate::UnitTest;

fn test_new_terminal_view() {
	//ASSERT: TerminalView::new()
	//returns a new TerminalView instance
	unimplemented!();
}

fn test_log_terminal_view() {
	//Visibility test: TerminalView
	//instance prints the given message
	//on ::log() call
	unimplemented!();
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
