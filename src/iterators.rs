use thiserror::Error;

use crate::spatial::Point;

pub trait ExtraIter: Iterator + Sized {
    /// Attempt to collect the iterator fallibly
    fn try_collecting<C>(self) -> Result<C, <C as TryFromIterator<Self>>::Error> where
        C: TryFromIterator<Self, Item=Self::Item>
    {
        C::try_from_iter(self)
    }

    /// Assert that the iterator yields a single element and return it
    fn single(mut self) -> Result<Self::Item, SingleError> {
        self
            .next()
            .map_or_else(|| Err(SingleError::None), |v| match self.next() {
                None => Ok(v),
                Some(_) => Err(SingleError::More)
            })
    }
}

impl<I: Iterator + Sized> ExtraIter for I {}

pub trait TryFromIterator<I>: Sized {
    type Item;
    type Error;

    fn try_from_iter(iter: I) -> Result<Self, Self::Error>;
}

/// An error for when an iterator did not yield _exactly_ one element
#[derive(Debug, Clone, Copy, Error, PartialEq, Eq, Hash)]
pub enum SingleError {
    #[error("Iterator yielded no elements")] None,
    #[error("Iterator yielded more than one element")] More
}

pub trait Enumerate2D: Sized + Iterator {
    type Item;
    type IntoIter: Iterator<Item=(Point<usize>, <Self as Enumerate2D>::Item)>;

    fn enumerate2d(self) -> Self::IntoIter;
}

impl<I> Enumerate2D for I where
    I: Iterator + Sized,
    I::Item: IntoIterator
{
    type Item = <I::Item as IntoIterator>::Item;
    type IntoIter = impl Iterator<Item=(Point<usize>, <Self as Enumerate2D>::Item)>;

    fn enumerate2d(self) -> Self::IntoIter {
        self
            .enumerate()
            .flat_map(|(y, row)| row
                .into_iter()
                .enumerate()
                .map(move |(x, item)| (Point { x, y }, item))
            )
    }
}

#[cfg(test)]
mod tests {
    use std::iter::{empty, once};

    use super::*;
    use itertools::assert_equal;
    use crate::spatial::Point;

    #[test]
    fn extra_iter_single() {
        assert_eq!(Ok(1), once(1).single());
        assert_eq!(Err(SingleError::None), empty::<()>().single());
        assert_eq!(Err(SingleError::More), [1, 2].into_iter().single());
    }

    #[test]
    fn enumerate2d() {
        assert_equal(
            [
                (Point::new(0, 0), 1), (Point::new(1, 0), 2),
                (Point::new(0, 1), 3), (Point::new(1, 1), 4)
            ],
            [[1, 2], [3, 4]].into_iter().enumerate2d()
        );
    }
}