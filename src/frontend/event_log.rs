/*!
 * Defines the EventLog struct.
 */
use std::rc::Rc;

use super::TerminalView;

/**
 * Stores events, and sends them to subscribing views for display.
 */
pub struct EventLog {
	_views: Vec<Rc<TerminalView>>,
}

impl EventLog {
	/**
	 * TODO
	 */
	pub fn new() -> EventLog {
		EventLog {
			_views: vec!()
		}
	}

	/**
	 * TODO
	 */
	pub fn subscribe(&mut self, subscribing_view: Rc<TerminalView>) {
		unimplemented!()
	}

	/**
	 * TODO
	 */
	pub fn log(&self, event: String) {
		//For each subscriber in _views:
		//	Call the subscriber's display function.
		for view in self._views.iter() {
			view.log(event.clone());
		}
	}
}