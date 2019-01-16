/*!
 * TODO
 */
use rtest_impl::frontend::{EventLog, TerminalView};
use rtest_impl::errors::ElementAddError;
use std::sync::Arc;

use crate::UnitTest;

fn test_new_event_log() {
	//ASSERT: EventLog::new()
	//returns a new EventLog instance
	let _ = EventLog::new();
}

fn test_subscribe() {
	let mut event_log = EventLog::new();
	let terminal_view = Arc::new(TerminalView::new());

	//ASSERT: Subscribing a new
	//subscriber succeeds.
	if let Err(e) = event_log.subscribe(terminal_view.clone()) {
		assert!(false, "Adding a new subscriber failed. Reason: {}", e);
	}

	//ASSERT: Subscribing an already subscribed
	//subscriber succeeds (returns Ok())
	//but doesn't add a duplicate subscriber
	let before_add_count = event_log.subscribers().len();
	let subscribe_result = event_log.subscribe(terminal_view.clone());
	if let Ok(ElementAddError::AlreadyInList { element_type: _, list_name: _ }) = subscribe_result {}
	else {
		let details = {
			match subscribe_result {
				Ok(x) => format!("succeeded instead with {}", x),
				Err(e) => format!("failed with {}", e)
			}
		};

		assert!(false, "Adding a existing subscriber should pass with ElementAddError::AlreadyInList, but {}", details);
	}


	let after_add_count = event_log.subscribers().len();
	assert!(before_add_count == after_add_count, "Adding an already subscribed subscriber should not modify the subscriber list. List len was {}, is now {} instead", before_add_count, after_add_count);
}

fn test_log_broadcast() {
	//Create *two* TerminalViews
	//so any logs print twice
	let mut event_log = EventLog::new();
	event_log.subscribe(Arc::new(TerminalView::new()));
	event_log.subscribe(Arc::new(TerminalView::new()));

	//Visibility test: broadcast the message,
	//it should appear twice
	println!("Running EventLog::log(); you should see two messages below.");
	event_log.log("Test message!");
}

/**
 * Returns all tests in this module.
 */
pub fn all_tests() -> Vec<UnitTest> {
	vec![
		unit_test!(test_new_event_log),
		unit_test!(test_subscribe),
		unit_test!(test_log_broadcast),
	]
}
