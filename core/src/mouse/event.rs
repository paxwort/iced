use crate::{Point, mouse::{TabletToolData, TabletToolKind, button::ButtonSource}, touch::Finger};


/// A mouse event.
///
/// _**Note:** This type is largely incomplete! If you need to track
/// additional events, feel free to [open an issue] and share your use case!_
///
/// [open an issue]: https://github.com/iced-rs/iced/issues
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    /// The mouse cursor entered the window.
    CursorEntered {
        /// The kind of pointer that entered
        kind: PointerKind,
        /// The new position of the mouse cursor
        position: Point,
    },
    /// The mouse cursor left the window.
    CursorLeft{
        /// The kind of pointer that left
        kind: PointerKind,
        /// The new position of the mouse cursor, or `None`
        position: Option<Point>,
    },

    /// The mouse cursor was moved
    CursorMoved {
        /// The pointer source that was moved
        source: PointerSource,
        /// The new position of the mouse cursor
        position: Point,
    },

    /// A mouse button was pressed.
    ButtonPressed{
        /// The source of the button press
        button: ButtonSource,
        /// The position of the mouse cursor
        position: Point,
    },

    /// A mouse button was released.
    ButtonReleased{
        /// The source of the button release
        button: ButtonSource,
        /// The position of the mouse cursor
        position: Point,
    },

    /// The mouse wheel was scrolled.
    WheelScrolled {
        /// The scroll movement.
        delta: ScrollDelta,
    },
}

/// A scroll movement.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScrollDelta {
    /// A line-based scroll movement
    Lines {
        /// The number of horizontal lines scrolled
        x: f32,

        /// The number of vertical lines scrolled
        y: f32,
    },
    /// A pixel-based scroll movement
    Pixels {
        /// The number of horizontal pixels scrolled
        x: f32,
        /// The number of vertical pixels scrolled
        y: f32,
    },
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PointerSource{
    Mouse,
    Touch(Finger),
    TabletTool{kind: TabletToolKind, data: TabletToolData},
    Unknown
}
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PointerKind{
    Mouse,
    Touch(Finger),
    TabletTool(TabletToolKind),
    Unknown
}



/// Shorthand to match a primary button press
#[macro_export]
macro_rules! pressed_primary{
    ($button:ident, $position:ident) => {
            $crate::mouse::Event::ButtonPressed{
            button: $button @ $crate::mouse::ButtonSource::Mouse($crate::mouse::Button::Left)
            | $button @ $crate::mouse::ButtonSource::TabletTool{ button: $crate::mouse::TabletToolButton::Contact, .. }
            | $button @ $crate::mouse::ButtonSource::Touch($crate::mouse::TouchButton::Primary(_)),
            position: $position
        }
    };
    () => {
        $crate::mouse::Event::ButtonPressed{
        button: $crate::mouse::ButtonSource::Mouse($crate::mouse::Button::Left)
        | $crate::mouse::ButtonSource::TabletTool{ button: $crate::mouse::TabletToolButton::Contact, .. }
        | $crate::mouse::ButtonSource::Touch($crate::mouse::TouchButton::Primary(_)),
        ..
        }
    };
}

/// Shorthand to match a secondary button press
#[macro_export]
macro_rules! pressed_secondary{
    ($button:ident, $position:ident) => {
            $crate::mouse::Event::ButtonPressed{
            button: $button @ $crate::mouse::ButtonSource::Mouse($crate::mouse::Button::Right)
            | $button @ $crate::mouse::ButtonSource::TabletTool{ button: $crate::mouse::TabletToolButton::Barrel, .. },
            position: $position
        }
    };
    () => {
        $crate::mouse::Event::ButtonPressed{
        button: $crate::mouse::ButtonSource::Mouse($crate::mouse::Button::Right)
        | $crate::mouse::ButtonSource::TabletTool{ button: $crate::mouse::TabletToolButton::Barrel, .. },
        ..
        }
    };
}

/// Shorthand to match an auxiliary button press (at the moment, that's just Middle Mouse Button)
#[macro_export]
macro_rules! pressed_auxiliary{
    ($button:ident, $position:ident) => {
            $crate::mouse::Event::ButtonPressed{
            button: $button @ $crate::mouse::ButtonSource::Mouse($crate::mouse::Button::Middle),
            position: $position
        }
    };
    () => {
        $crate::mouse::Event::ButtonPressed{
        button: $crate::mouse::ButtonSource::Mouse($crate::mouse::Button::Middle),
        ..
        }
    };
}


/// Shorthand to match a primary button release
#[macro_export]
macro_rules! released_primary{
    ($button:ident, $position:ident) => {
            $crate::mouse::Event::ButtonReleased{
            button: $button @ $crate::mouse::ButtonSource::Mouse($crate::mouse::Button::Left)
            | $button @ $crate::mouse::ButtonSource::TabletTool{ button: $crate::mouse::TabletToolButton::Contact, .. }
            | $button @ $crate::mouse::ButtonSource::Touch($crate::mouse::TouchButton::Primary(_)),
            position: $position
        }
    };
    () => {
        $crate::mouse::Event::ButtonReleased{
        button: $crate::mouse::ButtonSource::Mouse($crate::mouse::Button::Left)
        | $crate::mouse::ButtonSource::TabletTool{ button: $crate::mouse::TabletToolButton::Contact, .. }
        | $crate::mouse::ButtonSource::Touch($crate::mouse::TouchButton::Primary(_)),
        ..
        }
    };
}

/// Shorthand to match a secondary button release
#[macro_export]
macro_rules! released_secondary{
    ($button:ident, $position:ident) => {
            $crate::mouse::Event::ButtonReleased{
            button: $button @ $crate::mouse::ButtonSource::Mouse($crate::mouse::Button::Right)
            | $button @ $crate::mouse::ButtonSource::TabletTool{ button: $crate::mouse::TabletToolButton::Barrel, .. },
            position: $position
        }
    };
    () => {
        $crate::mouse::Event::ButtonReleased{
        button: $crate::mouse::ButtonSource::Mouse($crate::mouse::Button::Right)
        | $crate::mouse::ButtonSource::TabletTool{ button: $crate::mouse::TabletToolButton::Barrel, .. },
        ..
        }
    };
}

/// Shorthand to match an auxiliary button release (at the moment, that's just Middle Mouse Button)
#[macro_export]
macro_rules! released_auxiliary{
    ($button:ident, $position:ident) => {
            $crate::mouse::Event::ButtonReleased{
            button: $button @ $crate::mouse::ButtonSource::Mouse($crate::mouse::Button::Middle),
            position: $position
        }
    };
    () => {
        $crate::mouse::Event::ButtonReleased{
        button: $crate::mouse::ButtonSource::Mouse($crate::mouse::Button::Middle),
        ..
        }
    };
}
