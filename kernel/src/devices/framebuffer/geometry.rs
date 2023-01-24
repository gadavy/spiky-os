/// A 2d point.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    /// Creates a new `Point` at (x, y)
    pub const fn new(x: usize, y: usize) -> Self {
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

/// A rectangular region.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Rect {
    min: Point,
    max: Point,
}

impl Rect {
    /// Creates a new `Rect` from a position and size.
    pub const fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            min: Point::new(x, y),
            max: Point::new(x + width, y + height),
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

#[cfg(test)]
mod tests {
    use crate::devices::framebuffer::{Point, Rect};

    #[test]
    fn rect_contains() {
        let r = Rect::new(5, 5, 5, 5);
        assert!(r.contains(Point::new(5, 5)));
        assert!(r.contains(Point::new(10, 10)));
        assert!(!r.contains(Point::new(10, 11)));
        assert!(!r.contains(Point::new(11, 10)));
    }

    #[test]
    fn rect_intersection() {
        let r = Rect::new(5, 5, 5, 5);

        assert_eq!(
            r.intersection(Rect::new(5, 5, 2, 2)),
            Some(Rect::new(5, 5, 2, 2))
        );

        assert_eq!(
            r.intersection(Rect::new(2, 2, 5, 5)),
            Some(Rect::new(5, 5, 2, 2))
        );

        assert_eq!(r.intersection(Rect::new(11, 11, 5, 5)), None);
    }
}
