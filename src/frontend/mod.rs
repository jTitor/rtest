/*!
 * Defines the ```frontend``` module.
 */
mod terminal_view;
use self::terminal_view::*;

mod event_log;
use self::event_log::*;

mod frontend;
pub use self::frontend::*;