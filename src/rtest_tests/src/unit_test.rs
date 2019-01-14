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
		//Run it in catch_unwind.
		let test_result = panic::catch_unwind(self._test);
		match test_result {
			Ok(_) => Ok(()),
			Err(x) => {
				//If it failed,
				//try to report failure reason.
				let fail_reason: String = {
					if let Some(fail_string) = x.downcast_ref::<String>() {
						format!("{}", fail_string).into()
					} else if let Some(fail_string) = x.downcast_ref::<&str>() {
						format!("{}", fail_string).into()
					} else {
						format!("Couldn't get detailed failure string: {:?}", x).into()
					}
				};

				//Report test failed?
				Err(fail_reason)
			}
		}
	}
}

/**
 * Macro that creates UnitTest instances
 * for you, given a test function name.
 *
 * The UnitTest's name() will be whatever
 * function name you provided.
 */
macro_rules! unit_test {
	($test_name:ident) => {
		UnitTest::new(stringify!($test_name).to_string(), $test_name)
	};
}
