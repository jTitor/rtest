/*!
 * Defines functions and structs
 * used to handle panics during test execution.
 */

static mut LAST_PANIC_STRING: Option<String> = None;

/**
 * TODO_DESC
 */
fn clear_last_panic_string() {
	unsafe {
		LAST_PANIC_STRING = None;
	}
}

/**
 * TODO_DESC
 */
fn set_last_panic_string(string: &str) {
	unsafe {
		LAST_PANIC_STRING = Some(string.to_string());
	}
}

/**
 * Returns the info of the last panic as an Option<String>.
 */
pub fn get_last_panic_string() -> Option<String> {
	//Get the string
	let string = {
		unsafe {
			LAST_PANIC_STRING.clone()
		}
	};

	//Reset the stored string now
	clear_last_panic_string();

	string
}

/**
 * Sets the appropriate panic hook so that
 * get_last_panic_string() returns panic strings correctly.
 */
pub fn set_hook() {
	std::panic::set_hook(Box::new(|info| {
		//Stringify the whole info object.
		let cause = {
			//Can we stringify the payload?
			if let Some(payload_string) = info.payload().downcast_ref::<&str>() {
				//If so, do we have panic location data?
				if let Some(location) = info.location() {
					//If so, print full info
					format!(
						"{} at {}:{}",
						payload_string,
						location.file(),
						location.line()
					)
				} else {
					//Otherwise, just print the payload
					format!("{}", payload_string)
				}
			} else {
				//Other-otherwise, we have no info; print a placeholder message
				"unknown error".to_string()
			}
		};

		set_last_panic_string(cause.as_str());
	}));
}

/**
 * Unsets any panic hook set by panic_handling::set_hook().
 */
pub fn take_hook() {
	let _ = std::panic::take_hook();
}
