/*!
 * Defines the ```frontend``` module.
 */
mod terminal_view;
use self::terminal_view::*;

mod event_log;
use self::event_log::*;

mod frontend;
use self::frontend::*;
pub use self::frontend::Frontend;

mod static_frontend;
pub use self::static_frontend::*;
