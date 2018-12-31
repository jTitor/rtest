/*!
 * Defines the TerminalView struct.
 */

/**
 * Prints event data of an EventLog to standard output.
 */
pub struct TerminalView {

}

impl TerminalView {
	/**
	 * Returns a new TerminalView instance.
	 */
	pub fn new() -> TerminalView {
		TerminalView {}
	}

	pub fn log(&self, text: &str) {
		println!("{}", text);
		unimplemented!();
	}
}