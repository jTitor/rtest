/*!
 * TODO
 */
use crate::UnitTest;

fn test_new_event_log() {
	unimplemented!();
}

fn test_subscribe() {
	unimplemented!();
}

fn test_log_broadcast() {
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
