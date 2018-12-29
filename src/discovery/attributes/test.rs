/*!
 * Defines actual functionality of the #[test] attribute.
 */
use proc_macro::TokenStream;
use std::sync::MutexGuard;

use failure::Error;

use super::common;
use super::super::{StaticTestList, StaticTestListError, TestLists};


/**
 * TODO
 */
pub fn do_attribute_test(attr: &TokenStream, item: &TokenStream) {
	let attribute_name: String = "test".into();

	//Is item a function?
	if common::is_function(&item) {
		//If so:
		//Could we get the list instance?
		let mut list_result: Result<MutexGuard<TestLists>, Error> = Err(StaticTestListError::InstanceNotInitialized.into());
		unsafe {
			list_result = StaticTestList::instance();
		}
		if let Ok(mut list_mutex) = list_result {
			//	If so:
			//	Does attr contain "main"?
			let contains_main: bool = false;
			unimplemented!();
			if contains_main {	
					//If so:
					//Add item to the main test list.
					let main_test: u64 = 0;
					unimplemented!();
					list_mutex.add_main_test(main_test);
			}
			else {
					//Else:
					//Add item to the parallel test list.
					let test: u64 = 0;
					unimplemented!();
					list_mutex.add_test(test);
			}
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