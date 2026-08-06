use super::tablet_tool::{TabletToolButton, TabletToolData, TabletToolKind};

/// The pointer type of a button
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ButtonSource {
    /// The mouse
    Mouse {
        /// The button on the mouse
        button: MouseButton,
    },
    /// A tablet tool, such as a pen
    TabletTool {
        /// The button on the tablet tool
        button: TabletToolButton,
        /// The kind of tablet tool (pen, eraser, etc.)
        kind: TabletToolKind,
        /// The tablet tool data at this button press
        data: TabletToolData,
    },
}

/// The button of a mouse.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MouseButton {
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

    /// Some other button.
    Other(u16),
}
