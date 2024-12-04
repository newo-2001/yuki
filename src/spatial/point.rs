use num_traits::{Num, One, Zero};

use super::super::num::CheckedAddSigned;

use super::direction::Directions;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash,
    derive_more::Add, derive_more::Sub, derive_more::Neg
)]
pub struct Point<T = u32> {
    pub x: T,
    pub y: T
}

impl<T> Point<T> {
    pub fn neighbours<D>(self) -> impl Iterator<Item=Self> where
        T: Copy + Zero + One + CheckedAddSigned,
        D: Directions
    {
        D::all().filter_map(move |direction| self.add_signed(direction.vector()))
    }

    pub fn add_signed<U>(self, rhs: U) -> Option<Self> where
        T: CheckedAddSigned,
        U: Into<Point<T::Signed>>
    {
        let Self { x, y } = self;
        let Point::<T::Signed> { x: dx, y: dy } = rhs.into();

        Some(Self {
            x: x.checked_add_signed(dx)?,
            y: y.checked_add_signed(dy)?
        })
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