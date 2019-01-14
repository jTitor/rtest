/*!
 * TODO
 */
use crate::UnitTest;

fn test_new_event_log() {
	//ASSERT: EventLog::new()
	//returns a new EventLog instance
	unimplemented!();
}

fn test_subscribe() {
	//ASSERT: Subscribing a new
	//subscriber succeeds
	unimplemented!();

	//ASSERT: Subscribing a new
	//subscriber succeeds (returns Ok())
	//but doesn't add a duplicate subscriber
	unimplemented!();
}

fn test_log_broadcast() {
	//Create *two* TerminalViews
	//so any logs print twice
	unimplemented!();

	//Visibility test: broadcast the message,
	//it should appear twice
	unimplemented!();
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
