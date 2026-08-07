//! Handle mouse events.
pub mod click;

mod button;
mod cursor;
mod event;
mod interaction;
mod tablet_tool;

pub use tablet_tool::*;
pub use button::*;
pub use crate::{button_primary, button_secondary, button_auxiliary};
pub use click::Click;
pub use cursor::{Cursor, CursorSource};
pub use event::{Event, ScrollDelta};
pub use interaction::Interaction;
