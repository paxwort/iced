    #[allow(missing_docs)]
    #[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
    pub enum TabletToolButton {
        /// Primary contact button
        Contact,

        /// Barrel button
        Barrel,

        /// Some other button.
        Other(u16),
    }

    #[allow(missing_docs)]
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum TabletToolKind {
        Pen,
        Eraser,
        Brush,
        Pencil,
        Airbrush,
        Finger,
        Mouse,
        Lens,
    }

    #[allow(missing_docs)]
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct TabletToolData {
        pub force: Option<Force>,
        pub tangential_force: Option<f32>,
        pub twist: Option<u16>,
        pub tilt: Option<TabletToolTilt>,
        pub angle: Option<TabletToolAngle>,
    }


    #[allow(missing_docs)]
    /// The polar orientation of the tablet tool in relation to the tablet surface.
    /// Used by UIKit.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct TabletToolAngle {
        pub altitude: f64,
        pub azimuth: f64,
    }

    #[allow(missing_docs)]
    /// The cartesian orientation of the tablet tool in relation to the tablet surface.
    /// Used by everything else
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
    pub struct TabletToolTilt {
        pub x: i8,
        pub y: i8,
    }

    #[allow(missing_docs)]
    /// The force applied with the tablet tool
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Force {
        /// Calibrated force, used by UIKit.
        /// Do apple engineers get beaten if they implement something too close to existing solutions?
        Calibrated { force: f64, max_possible_force: f64 },
        /// Normalized force, used by everything else.
        Normalized(f64),
    }
