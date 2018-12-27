/*!
 * Defines the Frontend struct.
*/
use std::rc::Rc;

use super::{EventLog, TerminalView};

/**
 * Handles recording and reporting of events within this library.
 */
pub struct Frontend {
	_terminal_view: Rc<TerminalView>,
	_event_log: EventLog
}

impl Frontend {
	pub fn new() -> Frontend {
		//Make a TerminalView
		//Make an EventLog
		//Subscribe the TerminalView to the EventLog
		//Move them both into the Frontend instance
		unimplemented!()
	}
}