/*!
 * Defines errors used by the crate root
 * or modules below it.
 */
use failure::Fail;

/**
 * TODO
 */
#[derive(Debug, Fail)]
pub enum ElementAddError {
	#[fail(display = "successfully added {}", element_type)]
	Success { element_type: String },
	#[fail(display = "{} already contains {}", list_name, element_type)]
	AlreadyInList { element_type: String, list_name: String },
	#[fail(display = "{} could not be added to {} for unknown reason", element_type, list_name)]
	ListAppendFailed { element_type: String, list_name: String },
	#[fail(display = "{} could not be removed from {} for unknown reason", element_type, list_name)]
	ListRemoveFailed { element_type: String, list_name: String },
}
