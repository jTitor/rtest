/*!
 * Defines the FailureDetail struct.
*/

/**
 * Provides information as to how a given test failed.
 */
pub struct FailureDetail {
	test_name: String,
	failure_file: String,
	failure_line: u64,
	failure_backtrace: String,
}