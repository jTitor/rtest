/*!
 * Defines the TerminalView struct.
 */

/**
 * Prints event data of an EventLog to standard output.
 */
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct TerminalView {}

impl TerminalView {
	/**
	 * Returns a new TerminalView instance.
	 */
	pub fn new() -> TerminalView {
		TerminalView {}
	}

	/**
	 * Prints the given message
	 * to standard output.
	 */
	pub fn log(&self, text: &str) {
		println!("{}", text);
	}
}