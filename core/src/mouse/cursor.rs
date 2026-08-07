use crate::{Point, Rectangle, Transformation, Vector, mouse::TabletToolData};

/// The mouse cursor state.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Cursor {
    /// The cursor has a defined position.
    Available{
        /// Position of the cursor
        position: Point,
        /// Additional data from the cursor's pointer source
        source: Option<CursorSource>},

    /// The cursor has a defined position, but it's levitating over a layer above.
    Levitating{
        /// Position of the cursor
        position: Point,
        /// Additional data from the cursor's pointer source
        source: Option<CursorSource>},

    /// The cursor is currently unavailable (i.e. out of bounds or busy).
    #[default]
    Unavailable,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CursorSource{
    TabletTool(TabletToolData)
}

impl Cursor {

    /// Returns information associated with the [`Cursor`] source, if available.
    pub fn source(self) -> Option<CursorSource> {
        match self {
            Cursor::Available{ source: data, .. } => data,
            Cursor::Levitating{ .. } | Cursor::Unavailable => None,
        }
    }

    /// Returns the absolute position of the [`Cursor`], if available.
    pub fn position(self) -> Option<Point> {
        match self {
            Cursor::Available{ position, .. } => Some(position),
            Cursor::Levitating{ .. } | Cursor::Unavailable => None,
        }
    }

    /// Returns the absolute position of the [`Cursor`], if available and inside
    /// the given bounds.
    ///
    /// If the [`Cursor`] is not over the provided bounds, this method will
    /// return `None`.
    pub fn position_over(self, bounds: Rectangle) -> Option<Point> {
        self.position().filter(|p| bounds.contains(*p))
    }

    /// Returns the relative position of the [`Cursor`] inside the given bounds,
    /// if available.
    ///
    /// If the [`Cursor`] is not over the provided bounds, this method will
    /// return `None`.
    pub fn position_in(self, bounds: Rectangle) -> Option<Point> {
        self.position_over(bounds)
            .map(|p| p - Vector::new(bounds.x, bounds.y))
    }

    /// Returns the relative position of the [`Cursor`] from the given origin,
    /// if available.
    pub fn position_from(self, origin: Point) -> Option<Point> {
        self.position().map(|p| p - Vector::new(origin.x, origin.y))
    }

    /// Returns true if the [`Cursor`] is over the given `bounds`.
    pub fn is_over(self, bounds: Rectangle) -> bool {
        self.position_over(bounds).is_some()
    }

    /// Returns true if the [`Cursor`] is levitating over a layer above.
    pub fn is_levitating(self) -> bool {
        matches!(self, Self::Levitating{ .. })
    }

    /// Makes the [`Cursor`] levitate over a layer above.
    pub fn levitate(self) -> Self {
        match self {
            Self::Available{ position, source } => Self::Levitating{ position, source },
            _ => self,
        }
    }

    /// Brings the [`Cursor`] back to the current layer.
    pub fn land(self) -> Self {
        match self {
            Cursor::Levitating{ position, source } => Cursor::Available{ position, source },
            _ => self,
        }
    }
}

impl std::ops::Add<Vector> for Cursor {
    type Output = Self;

    fn add(self, translation: Vector) -> Self::Output {
        match self {
            Cursor::Available{ position, source } => Cursor::Available{ position: position + translation, source},
            Cursor::Levitating{ position, source } => Cursor::Levitating{position: position + translation, source},
            Cursor::Unavailable => Cursor::Unavailable,
        }
    }
}

impl std::ops::Sub<Vector> for Cursor {
    type Output = Self;

    fn sub(self, translation: Vector) -> Self::Output {
        match self {
            Cursor::Available{ position, source } => Cursor::Available{ position: position - translation, source},
            Cursor::Levitating{ position, source } => Cursor::Levitating{ position: position - translation, source},
            Cursor::Unavailable => Cursor::Unavailable,
        }
    }
}

impl std::ops::Mul<Transformation> for Cursor {
    type Output = Self;

    fn mul(self, transformation: Transformation) -> Self {
        match self {
            Self::Available{ position, source } => Self::Available{ position: position * transformation, source},
            Self::Levitating{ position, source } => Self::Levitating{ position: position * transformation, source},
            Self::Unavailable => Self::Unavailable,
        }
    }
}
