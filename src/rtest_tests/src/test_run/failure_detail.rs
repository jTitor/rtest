/*!
 * TODO
 */
use rtest_impl::test_run::FailureDetail;

use crate::UnitTest;

fn test_parse_from_string() {
	//Generate the detail.
	let expected_test_name = "Test Name";
	let test_details_content = "Example Details Content";
	let detail = FailureDetail::parse_from_string(expected_test_name,
	test_details_content);

	//Get the components now that they're
	//parsed.
	let actual_test_name = detail.test_name;
	let actual_failure_file = detail.failure_file;
	let actual_failure_line = detail.failure_line;
	let actual_failure_reason = detail.failure_reason;
	let actual_failure_backtrace = detail.failure_backtrace;

	//ASSERT: All of the important components
	//match their expected values.
	//Since FailureDetail::parse_failure_string()
	//is only placeholder-implemented,
	//only the failure reason should have meaningful data
	assert!(actual_test_name == expected_test_name, "Test name should be {}, is {}", expected_test_name, actual_test_name);
	
	let expected_failure_file = "TODO";
	assert!(actual_failure_file == expected_failure_file, "Failure file should be {}, is {}", expected_failure_file, actual_failure_file);

	let expected_failure_line = 0;
	assert!(actual_failure_line == expected_failure_line, "Failure line should be {}, is {}", expected_failure_line, actual_failure_line);

	let expected_failure_reason = test_details_content;
	assert!(actual_failure_reason == expected_failure_reason, "Failure reason should be {}, is {}", expected_failure_reason, actual_failure_reason);

	let expected_failure_backtrace = "TODO";
	assert!(actual_failure_backtrace == expected_failure_backtrace, "Failure backtrace should be {}, is {}", expected_failure_backtrace, actual_failure_backtrace);
}

/**
 * Returns all tests in this module.
 */
pub fn all_tests() -> Vec<UnitTest> {
	vec![unit_test!(test_parse_from_string)]
}