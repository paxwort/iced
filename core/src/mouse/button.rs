use super::tablet_tool::{TabletToolButton, TabletToolData, TabletToolKind};
/// The source of a button.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ButtonSource {
    /// The mouse
    Mouse( Button ),
    #[allow(missing_docs)]
    /// A tablet tool, such as a pen
    TabletTool {
        button: TabletToolButton,
        kind: TabletToolKind,
        data: TabletToolData },
    /// "Button" sources from other touch events
    Touch (TouchButton),
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
