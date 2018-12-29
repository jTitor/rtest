/*!
 * TODO
 */
use crate::UnitTest;

fn test_runner_counts() {
	//Setup test list.
	unimplemented!();

	//Init runner.
	unimplemented!();

	//Run test list.
	unimplemented!();

	//ASSERT: passed test count matches expected count.
	unimplemented!();
	//ASSERT: failed test count matches expected count.
	unimplemented!();
	//ASSERT: ignored test count matches expected count.
	unimplemented!();

	//ASSERT: total test count matches expected count,
	//the sum of the previous expected counts.
	unimplemented!();

	//ASSERT: number of test failure details equals
	//failed test count.
	unimplemented!();
}

fn test_runner_result() {
	//Setup test lists.
	unimplemented!();

	//Init runner.
	unimplemented!();

	//ASSERT: runner returns Ok() when
	//all tests pass.
	unimplemented!();

	//ASSERT: runner returns Ok() when
	//all tests are skipped.
	unimplemented!();

	//ASSERT: runner returns Err() when
	//any test fails.
	unimplemented!();

	//ASSERT: runner returns Err() when
	//all tests fail.
	unimplemented!();
}

/**
 * Returns all tests in this module.
 */
pub fn all_tests() -> Vec<UnitTest> {
	vec![unit_test!(test_runner_counts), unit_test!(test_runner_result)]
}
