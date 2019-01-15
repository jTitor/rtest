/*!
 * Defines functions used by multiple tests
 * in this module.
 */
use rtest_impl::discovery::TestEntry;

/**
 * Dummy function that does nothing,
 * to fill out the TestEntry instances in test_modify_lists().
 */
pub fn dummy_fn() {}

/**
 * Creates a test entry with the given name,
 * using the dummy function dummy_fn.
 */
pub fn test_entry_named(name: &str) -> TestEntry {
	TestEntry {
		name: name.into(),
		test: dummy_fn,
	}
}