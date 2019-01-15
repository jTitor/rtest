/*!
 * TODO
 */
use rtest_impl::frontend::{EventLog, TerminalView};
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
	//subscriber succeeds
	unimplemented!();
	event_log.subscribe(terminal_view.clone());

	//ASSERT: Subscribing an already subscribed
	//subscriber succeeds (returns Ok())
	//but doesn't add a duplicate subscriber
	event_log.subscribe(terminal_view.clone());
	unimplemented!();
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
