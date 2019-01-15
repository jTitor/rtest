/*!
 * Defines actual functionality of the #[test] attribute.
 */
use proc_macro::TokenStream;
use std::sync::MutexGuard;

use failure::Error;

use super::super::{StaticTestList, StaticTestListError, TestLists};
use crate::discovery::attributes::common;
use crate::discovery::rls_common;

/**
 * TODO: We probably actually
 * *remove* this, since our generators
 * in ::discovery::test_harness_root handle
 * function discovery and codegen
 */
pub fn do_attribute_test(attr: &TokenStream, item: &TokenStream) {
	let attribute_name: String = "test".into();

	//Is item a function?
	if common::is_function(item) {
		//If so:
		//Could we get the list instance?
		let mut list_result: Result<MutexGuard<TestLists>, Error> =
			Err(StaticTestListError::InstanceNotInitialized.into());
		unsafe {
			list_result = StaticTestList::instance();
		}

		if let Ok(mut list_mutex) = list_result {
			let test_entry = common::extract_test_entry(item);
			let test_entry_name = test_entry.name.clone();
			//	If so:
			//	Does attr contain "main"?
			let contains_main: bool = common::attr_contains_main(attr);
			if contains_main {
				//If so:
				//Add item to the main test list.
				if let Err(e) = list_mutex.add_main_test(test_entry) {
					//Warn if this failed to add.
					common::debug_warn_test_add_failed(
						test_entry_name.as_str(),
						format!("{}", e).as_str(),
					);
				}
			} else {
				//Else:
				//Add item to the parallel test list.
				if let Err(e) = list_mutex.add_test(test_entry) {
					//Warn if this failed to add.
					common::debug_warn_test_add_failed(
						test_entry_name.as_str(),
						format!("{}", e).as_str(),
					);
				}
			}
		}
		//Else:
		else {
			//Report failure.
			common::warn_list_instance_failed(&attribute_name);
		}
	} else {
		//Else:
		//	Warn that the item isn't a function and that this tag has no effect.
		common::warn_not_function(item, &attribute_name);
	}
}
