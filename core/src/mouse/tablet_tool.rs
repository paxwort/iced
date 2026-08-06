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
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
    pub struct TabletToolTilt {
        pub x: i8,
        pub y: i8,
    }

    #[allow(missing_docs)]
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct TabletToolAngle {
        pub altitude: f64,
        pub azimuth: f64,
    }

    #[allow(missing_docs)]
    // Device-calibrated force because Apple couldn't stand the idea of user calibration I guess?
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Force {
        Calibrated { force: f64, max_possible_force: f64 },
        Normalized(f64),
    }
