/*!
 * Defines the FailureDetail struct.
*/
use std::fmt;

/**
 * Provides information as to how a given test failed.
 */
#[derive(Debug, Default, Clone)]
pub struct FailureDetail {
	pub test_name: String,
	pub failure_file: String,
	pub failure_line: u64,
	pub failure_backtrace: String,
	pub failure_reason: String,
}

/**
 * Parses the given raw failure string
 * into its file, line, backtrace, and
 * failure reason components.
 * 
 * Returns a tuple in the following order:
 * 	1: failure file
 * 	2: failure line number
 * 	3: failure backtrace, if any
 * 	4: failure reason
*/
fn parse_failure_string(failure_raw_string: &str) -> (String, u64, String, String) {
	//TODO: implement actual parsing
	//once we have output from tests
	(
		"TODO".to_string(),
		Default::default(),
		"TODO".to_string(),
		failure_raw_string.to_string(),
	)
}

impl FailureDetail {
	/**
	 * Parses a FailureDetail from
	 * a raw string.
	 */
	pub fn parse_from_string(test_name: &str, failure_raw_string: &str) -> FailureDetail {
		let (failure_file, failure_line, failure_backtrace, failure_reason) =
			parse_failure_string(failure_raw_string);

		FailureDetail {
			test_name: test_name.to_string(),
			failure_file: failure_file,
			failure_line: failure_line,
			failure_backtrace: failure_backtrace,
			failure_reason: failure_reason,
		}
	}

	/**
	 * Creates a FailureDetail, default-initializing
	 * everything except the test name
	 * and failure reason. The test name
	 * will be in test_name, while the 
	 * failure_reason will contain the full
	 * raw string passed to this function.
	 */
	pub fn from_raw_string(test_name: &str, failure_raw_string: &str) -> FailureDetail {
		FailureDetail {
			test_name: test_name.to_string(),
			failure_file: Default::default(),
			failure_line: Default::default(),
			failure_backtrace: Default::default(),
			failure_reason: failure_raw_string.to_string(),
		}
	}
}

impl fmt::Display for FailureDetail {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Test '{}' failed at {}:{}. Reason: '{}'. Backtrace: {}", self.test_name, self.failure_file, self.failure_line, self.failure_reason, self.failure_backtrace)
	}
}
