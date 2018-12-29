/*!
 * Defines actual functionality of the #[ignore] attribute.
 */
use proc_macro::TokenStream;
use std::sync::MutexGuard;

use failure::Error;

use super::common;
use super::super::{StaticTestList, StaticTestListError, TestLists};

pub fn do_attribute_ignore(_attr: &TokenStream, item: &TokenStream) {
	let attribute_name: String = "ignore".into();

	//Is item a function?
	if common::is_function(&item) {
		//If so:
		//Could we get the list instance?
		let mut list_result: Result<MutexGuard<TestLists>, Error> = Err(StaticTestListError::InstanceNotInitialized.into());
		unsafe {
			list_result = StaticTestList::instance();
		}
		if let Ok(mut list_mutex) = list_result {
			//If so:
			//	Add item to the ignore list.
			let test: u64 = 0;
			list_mutex.ignore_test(test);
		}
		//Else:
		else {
			//Report failure.
			common::warn_list_instance_failed(attribute_name);
		}
	}
	else {
		//Else:
		//	Warn that the item isn't a function and that this tag has no effect.
		common::warn_not_function(&item, attribute_name);
	}
}