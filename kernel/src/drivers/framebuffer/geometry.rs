/// A 2d point.
#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    /// Creates a new `Point` at (x, y)
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// Returns the x/horizontal component of the point.
    #[inline]
    pub fn x(self) -> usize {
        self.x
    }

    /// Returns the y/vertical component of the point.
    #[inline]
    pub fn y(self) -> usize {
        self.y
    }
}

impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl From<[usize; 2]> for Point {
    fn from(value: [usize; 2]) -> Self {
        Self::new(value[0], value[1])
    }
}

/// A rectangular region of non-zero width and height.
#[derive(Debug, Copy, Clone)]
pub struct Rect {
    min: Point,
    max: Point,
}

impl Rect {
    /// Creates a new `Rect` from a position and size. Width and height
    /// are required to be strictly positive.
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        assert!(width > 0, "width must be strictly positive");
        assert!(height > 0, "height must be strictly positive");

        Self {
            min: Point::new(x, y),
            max: Point::new(x + width - 1, y + height - 1),
        }
    }

    /// Returns the minimum `Point` of the `Rect`.
    #[inline]
    pub fn min(self) -> Point {
        self.min
    }

    /// Returns the maximum `Point` of the `Rect`.
    #[inline]
    pub fn max(self) -> Point {
        self.max
    }

    /// Returns the width of the `Rect`.
    #[inline]
    pub fn width(self) -> usize {
        self.max().x() - self.min().x()
    }

    /// Returns the height of the `Rect`.
    #[inline]
    pub fn height(self) -> usize {
        self.max().y() - self.min().y()
    }

    /// Whether this `Rect` contains the given `Point`.
    #[inline]
    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
    }

    /// Returns the intersection of self and other, or none if they are are disjoint.
    pub fn intersection(self, other: Self) -> Option<Self> {
        let left = core::cmp::max(self.min.x, other.min.x);
        let top = core::cmp::max(self.min.y, other.min.y);
        let right = core::cmp::min(self.max.x, other.max.x);
        let bottom = core::cmp::min(self.max.y, other.max.y);

        if right < left || bottom < top {
            return None;
        }

        Some(Rect {
            min: Point::new(left, top),
            max: Point::new(right, bottom),
        })
    }
}
