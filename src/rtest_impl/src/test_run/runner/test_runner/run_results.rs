/*!
 * TODO
*/
use std::fmt;

use crate::test_run::FailureDetail;

/**
 * TODO
 */
#[derive(Debug, Clone)]
pub struct RunResults {
	pub failures: Vec<FailureDetail>,
	pub pass_count: u64,
	pub fail_count: u64,
	pub ignore_count: u64,
	pub tests_evaluated_count: u64,
}

impl RunResults {
	pub fn new() -> RunResults {
		RunResults {
			failures: vec![],
			pass_count: 0,
			fail_count: 0,
			ignore_count: 0,
			tests_evaluated_count: 0,
		}
	}
}

impl fmt::Display for RunResults {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "(\npass_count: {},\nfail_count: {},\nignore_count: {},\ntests_evaluated_count: {})", self.pass_count, self.fail_count, self.ignore_count, self.tests_evaluated_count)
	}
}
