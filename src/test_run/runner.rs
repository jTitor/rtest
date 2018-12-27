/*!
 * Defines the TestRunner struct.
*/
use super::FailureDetail;

/**
 * Given a TestLists instance, runs the different tests as needed.
 */
pub struct TestRunner {
	_failures: Vec<FailureDetail>,
	_pass_count: u64,
	_fail_count: u64,
	_ignore_count: u64,
}

impl TestRunner {
	pub fn run(&mut self) {
		//Clear all fields.
		self._failures.clear();
		self._pass_count = 0;
		self._fail_count = 0;
		self._ignore_count = 0;

		//The order of this may change.
		//For each test in ignored_tests:
		//	Report that the test is ignored to frontend.
		//	Increment ignored counter accordingly.
		//Iterate over parallel_tests:
		//	Create a list of N test failure details: "test_failures" (Option<FailureDetail>?)
		//	Get N tests, where N = number of cores on system
		//	Create a job for each test:
		//		Report that the test is starting to frontend.
		//		Run the test.
		//		Save any test failure details to test_failure[N].
		//		Report pass/fail to frontend.
		//	Copy any Some<FailureDetail> in test_failures to self._failures, increment failure counter accordingly.
		//	Increment run counter accordingly.
		//	Increment pass counter (?) accordingly.
		//For each test in main_tests:
		//	Report that the test is starting to frontend.
		//	Run the test.
		//	Save any test failure details to self._failures list.
		//	Report pass/fail to frontend.
		//	Increment failure counter accordingly.
		//	Increment run counter accordingly.
		//	Increment pass counter (?) accordingly.
		unimplemented!()
	}
}