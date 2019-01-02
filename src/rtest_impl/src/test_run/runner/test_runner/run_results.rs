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
	_failures: Vec<FailureDetail>,
	_pass_count: u64,
	_ignore_count: u64,
}

impl RunResults {
	/**
	 * TODO
	 */
	pub fn new() -> RunResults {
		RunResults {
			_failures: vec![],
			_pass_count: 0,
			_ignore_count: 0,
		}
	}

	/**
	 * TODO
	 */
	pub fn failure_details(&self) -> &Vec<FailureDetail> {
		&self._failures
	}

	pub fn failure_count(&self) -> u64 {
		self._failures.len()
	}

	/**
	 * TODO
	 */
	pub fn add_failure(&mut self, failure: FailureDetail) {
		self._failures.push(failure);
	}

	/**
	 * TODO
	 */
	pub fn pass_count(&self) -> u64 {
		self._pass_count
	}

	/**
	 * TODO
	 */
	pub fn add_pass(&mut self) {
		self._pass_count += 1;
	}

	/**
	 * TODO
	 */
	pub fn ignore_count(&self) -> u64 {
		self._ignore_count
	}

	/**
	 * TODO
	 */
	pub fn add_ignore(&mut self) {
		self._ignore_count += 1;
	}

	pub fn tests_evaluated_count(&self) -> u64 {
		self.failure_count() + self.pass_count() + self.ignore_count()
	}
}

impl fmt::Display for RunResults {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "(\npass_count: {},\nfail_count: {},\nignore_count: {},\ntests_evaluated_count: {})", self.pass_count(), self.fail_count(), self.ignore_count(), self.tests_evaluated_count())
	}
}
