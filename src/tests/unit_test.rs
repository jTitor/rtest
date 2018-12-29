/*!
 * Defines the UnitTest struct.
 */
use std::panic;

/**
 * Defines basic info of a unit test.
 */
pub struct UnitTest {
	_test_name: String,
	_test: fn(),
}

impl UnitTest {
	/**
	 * Creates a new UnitTest instance.
	 */
	pub fn new(test_name: String, test: fn()) -> UnitTest {
		UnitTest {
			_test_name: test_name,
			_test: test,
		}
	}

	pub fn name(&self) -> &String {
		&self._test_name
	}

	/**
	 * Runs the unit test.
	 * Returns Ok if and only if
	 * no errors, asserts or panics happened
	 * inside the unit test's test function.
	 */
	pub fn run(&self) -> Result<(), String> {
		println!("Running test '{}'", self._test_name);

		//Run it in catch_unwind.
		let test_result = panic::catch_unwind(self._test);
		match test_result {
			Ok(_) => {
				Ok(())
			}
			Err(x) => {
				let mut fail_reason: String = "".into();

				//If it failed,
				//try to report failure reason.
				if let Some(fail_string) = x.downcast_ref::<String>() {
				fail_reason = format!("{}", fail_string).into();
			}
			else if let Some(fail_string) = x.downcast_ref::<&str>() {
				fail_reason = format!("{}", fail_string).into();
			}
			else {
				fail_reason = format!("Couldn't get detailed failure string: {:?}", x).into();
			}
				//Report test failed?
				Err(fail_reason)
			}
		}
	}
}