/*!
 * Defines the EventLog struct.
 */
use std::sync::Arc;

use failure::Error;

use super::TerminalView;
use crate::errors::ElementAddError;

fn subscriber_list_name() -> String {
	"subscriber list".into()
}
fn subscriber_list_element_name() -> String {
	"subscriber".into()
}

/**
 * Stores events, and sends them to subscribing views for display.
 */
pub struct EventLog {
	_views: Vec<Arc<TerminalView>>,
}

impl EventLog {
	/**
	 * Creates a new EventLog instance.
	 */
	pub fn new() -> EventLog {
		EventLog { _views: vec![] }
	}

	/**
	 * Adds the given subscribing view
	 * to the event log system.
	 *
	 * Returns Ok if the subscriber was added
	 * or already in the EventLog's subscriber list,
	 * Err otherwise.
	 */
	pub fn subscribe(
		&mut self,
		subscribing_view: Arc<TerminalView>,
	) -> Result<ElementAddError, Error> {
		//Does this view already exist in the list?
		let found_instances = self
			._views
			.iter()
			.filter(|&x| *x == subscribing_view)
			.collect::<Vec<_>>();
		if found_instances.len() > 0 {
			//If so, early out
			return Ok(ElementAddError::AlreadyInList {
				element_type: subscriber_list_element_name(),
				list_name: format!("{}", subscriber_list_name()),
			});
		}

		//Otherwise, add it
		self._views.push(subscribing_view.clone());

		Ok(ElementAddError::Success {
			element_type: subscriber_list_element_name(),
		})
	}

	/**
	 * Returns the list of subscribers to this EventLog.
	 */
	pub fn subscribers(&self) -> &Vec<Arc<TerminalView>> {
		&self._views
	}

	/**
	 * Prints the given log message to all subscribers.
	 */
	pub fn log(&self, event: &str) {
		//For each subscriber in _views:
		//	Call the subscriber's display function.
		for view in self._views.iter() {
			view.log(event);
		}
	}
}
