/*!
 * TODO
 */

mod file_module;

pub mod pub_file_module;
pub use crate::pub_file_module::shortcut_import;

mod folder_module;
use crate::folder_module::*;

fn main() {
	println!("Hello, world!");
}
