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
	_event_log: EventLog,
}

impl Frontend {
	pub fn new() -> Frontend {
		//Make a TerminalView
		let terminal_view = Rc::new(TerminalView::new());
		//Make an EventLog
		let mut event_log = EventLog::new();

		//Subscribe the TerminalView to the EventLog
		event_log.subscribe(terminal_view.clone());

		//Move them both into the Frontend instance
		Frontend {
			_terminal_view: terminal_view,
			_event_log: event_log,
		}
	}

	/**
	 * TODO
	 */
	pub fn log(&self, event: String) {
		self._event_log.log(event);
	}
}