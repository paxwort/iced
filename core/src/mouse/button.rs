use super::tablet_tool::{TabletToolButton, TabletToolData, TabletToolKind};
/// The source of a button.
/// Also provides opinionated interpretation of buttons based on the language at https://www.w3.org/TR/pointerevents/
///
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ButtonSource {
    /// The mouse
    Mouse( Button ),
    /// A tablet tool, such as a pen
    TabletTool ( TabletToolButton ),
    /// "Button" sources from other touch events
    Touch (TouchButton),
}


/// Shorthand to match a primary button on any pointer source
#[macro_export]
macro_rules! button_primary{
    () => {
        $crate::mouse::ButtonSource::Mouse($crate::mouse::Button::Left)
        | $crate::mouse::ButtonSource::TabletTool($crate::mouse::TabletToolButton::Contact)
        | $crate::mouse::ButtonSource::Touch($crate::mouse::TouchButton::Primary(_))
    };
}

/// Shorthand to match a secondary button on any pointer source
#[macro_export]
macro_rules! button_secondary{
    () => {
        $crate::mouse::ButtonSource::Mouse($crate::mouse::Button::Right)
        | $crate::mouse::ButtonSource::TabletTool($crate::mouse::TabletToolButton::Barrel)
    };
}

/// Shorthand to match an auxiliary button on any pointer source (at the moment, that's just Middle Mouse Button)
#[macro_export]
macro_rules! button_auxiliary{
    () => {
        $crate::mouse::ButtonSource::Mouse($crate::mouse::Button::Middle)
    };
}

/// A touch event
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TouchButton{
    ///The first of all current touching fingers
    Primary(crate::touch::Finger),
    ///All other fingers
    Other(crate::touch::Finger)
}


/// The button of a mouse.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Button {
    /// The left mouse button.
    Left,

    /// The right mouse button.
    Right,

    /// The middle (wheel) button.
    Middle,

    /// The back mouse button.
    Back,

    /// The forward mouse button.
    Forward,

    /// Some other mouse button.
    Other(u16),
}
