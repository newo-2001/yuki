use std::cmp::{minmax, Ordering};
use std::ops::{Add, Sub};

use num_traits::{Num, One, Zero};

use super::super::num::CheckedAddSigned;

use super::direction::Directions;

/// Represents a point in 2D space
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default,
    derive_more::Add, derive_more::Sub, derive_more::Neg
)]
pub struct Point<T = usize> {
    pub x: T,
    pub y: T
}

impl<T> Point<T> {
    /// Creates a new [`Point`] with the corresponding `x` and `y` components
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// Converts from [`Point<T>`] to [`Point<U>`]
    /// 
    /// Returns [`None`] if the conversion is not possible
    pub fn cast<U>(self) -> Option<Point<U>> where
        T: TryInto<U>
    {
        Some(Point {
            x: self.x.try_into().ok()?,
            y: self.y.try_into().ok()?
        })
    }

    /// Creates an iterator over all the neighbours of `self`
    /// in all `D` directions which are representable by `T`
    pub fn neighbours<D>(self) -> impl Iterator<Item=Self> where
        T: Copy + Zero + One + CheckedAddSigned,
        D: Directions
    {
        D::all().filter_map(move |direction| self.add_signed(direction.vector()))
    }

    /// Attempts to add a signed [`Point<U>`] to `self`,
    /// 
    /// returns [`None`] if the result is not a valid `T`
    pub fn add_signed<U>(self, rhs: U) -> Option<Self> where
        T: CheckedAddSigned,
        U: Into<Point<T::Signed>>
    {
        let Self { x, y } = self;
        let Point { x: dx, y: dy } = rhs.into();

        Some(Self {
            x: x.checked_add_signed(dx)?,
            y: y.checked_add_signed(dy)?
        })
    }

    #[must_use]
    /// Returns the `(0, 0)` (origin) [`Point`]
    pub fn zero() -> Self where
        T: Zero
    {
        Self { x: T::zero(), y: T::zero() }
    }

    #[must_use]
    /// Returns the `(1, 1)` (unit) [`Point`]
    pub fn one() -> Self where
        T: One
    {
        Self { x: T::one(), y: T::one() }
    }

    #[must_use]
    /// Computes the manhatten distance of `self` to `other`
    /// 
    /// The manhattan distance is the sum of the absolute differences
    /// of the components of the points
    pub fn manhattan_distance(self, other: Self) -> T where
        T: Ord + Sub<Output=T> + Add<Output=T>
    {
        let [min_x, max_x] = minmax(self.x, other.x);
        let [min_y, max_y] = minmax(self.y, other.y);

        max_x - min_x + max_y - min_y
    }
}

impl<T> From<(T, T)> for Point<T> {
    fn from((x, y): (T, T)) -> Self {
        Self { x, y }
    }
}

impl<T> From<Point<T>> for (T, T) {
    fn from(val: Point<T>) -> Self {
        (val.x, val.y)
    }
}

impl<T> PartialOrd for Point<T> where
    T: PartialOrd
{
    /// The product order of the points on the cartesian plane
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let x = self.x.partial_cmp(&other.x)?;
        let y = self.y.partial_cmp(&other.y)?;

        match (x, y) {
            (Ordering::Equal, order) | (order, Ordering::Equal) => Some(order),
            (_, _) => None
        }
    }
}

macro_rules! impl_scalar_op {
    ($trait:ident, $function:ident, $operator:tt) => {
        impl<T> std::ops::$trait<T> for Point<T> where
            T: std::ops::$trait<Output=T> + Num + Copy
        {
            type Output = Self;

            fn $function(self, rhs: T) -> Self::Output {
                Self {
                    x: self.x $operator rhs,
                    y: self.y $operator rhs
                }
            }
        }
    }
}

impl_scalar_op!(Add, add, +);
impl_scalar_op!(Sub, sub, -);
impl_scalar_op!(Mul, mul, *);
impl_scalar_op!(Div, div, /);

#[cfg(test)]
mod tests {
    use itertools::assert_equal;
    use crate::spatial::direction;
    use super::*;

    #[test]
    fn point_manhattan_distance() {
        assert_eq!(8, Point::new(5, 2).manhattan_distance(Point::new(1, -2)));
        assert_eq!(0, Point::zero().manhattan_distance(Point::zero()));
    }

    #[test]
    fn point_neighbours() {
        assert_equal(
            [Point::new(1, 0), Point::new(0, 1)],
            Point::<u32>::new(0, 0).neighbours::<direction::Cardinal>()
        );
    }

    #[test]
    fn point_cast() {
        assert_eq!(Some(Point::<usize>::new(1, 0)), Point::<isize>::new(1, 0).cast::<usize>());
        assert_eq!(None, Point::<u8>::new(255, 0).cast::<i8>());
    }

    #[test]
    fn point_order() {
        assert_eq!(None, Point::new(0, 1).partial_cmp(&Point::new(1, 0)));
        assert_eq!(Some(Ordering::Equal), Point::new(1, 1).partial_cmp(&Point::new(1, 1)));
        assert_eq!(Some(Ordering::Greater), Point::new(0, 1).partial_cmp(&Point::new(0, 0)));
        assert_eq!(Some(Ordering::Less), Point::new(0, 0).partial_cmp(&Point::new(1, 0)));
    }
}