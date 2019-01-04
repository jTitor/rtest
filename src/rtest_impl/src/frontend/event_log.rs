/*!
 * Defines the EventLog struct.
 */
use std::sync::Arc;

use super::TerminalView;

/**
 * Stores events, and sends them to subscribing views for display.
 */
pub struct EventLog {
	_views: Vec<Arc<TerminalView>>,
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
	pub fn subscribe(&mut self, subscribing_view: Arc<TerminalView>) {
		self._views.push(subscribing_view.clone());
	}

	/**
	 * TODO
	 */
	pub fn log(&self, event: &str) {
		//For each subscriber in _views:
		//	Call the subscriber's display function.
		for view in self._views.iter() {
			view.log(event);
		}
	}
}