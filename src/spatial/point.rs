use derive_more::{Add, Sub};
use num_traits::{One, Zero};

use super::super::num::CheckedAddSigned;

use super::direction::Directions;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Add, Sub)]
pub struct Point<T = u32> {
    pub x: T,
    pub y: T
}

impl<T> Point<T> {
    pub fn neighbours<D>(self) -> impl Iterator<Item=Self> where
        T: Copy + Zero + One + CheckedAddSigned,
        D: Directions
    {
        D::all().filter_map(move |direction| self + direction)
    }
}

impl<T, D> Add<D> for Point<T> where
    T: Zero + One + CheckedAddSigned,
    D: Directions
{
    type Output = Option<Self>;
    
    fn add(self, rhs: D) -> Self::Output {
        let (dx, dy) = rhs.vector::<T::Signed>();
        let Self { x, y } = self;

        Some(Self {
            x: x.checked_add_signed(dx)?,
            y: y.checked_add_signed(dy)?
        })
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