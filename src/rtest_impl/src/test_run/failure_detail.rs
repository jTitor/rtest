/*!
 * Defines the FailureDetail struct.
*/

/**
 * Provides information as to how a given test failed.
 */
#[derive(Debug, Default, Clone)]
pub struct FailureDetail {
	test_name: String,
	failure_file: String,
	failure_line: u64,
	failure_backtrace: String,
	failure_reason: String,
}

/**
 * TODO
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
	 * TODO
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
	 * TODO
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
