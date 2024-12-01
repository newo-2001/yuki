use thiserror::Error;

pub trait ExtraIter: Iterator + Sized {
    fn try_collecting<C>(self) -> Result<C, <C as TryFromIterator<Self>>::Error> where
        C: TryFromIterator<Self, Item=Self::Item>
    {
        C::try_from_iter(self)
    }

    fn min_max(self) -> Option<(Self::Item, Self::Item)>
        where Self::Item: Ord + Copy
    {
        self.fold(None, |acc, x| {
            if let Some((min, max)) = acc {
                Some((x.min(min), x.max(max)))
            } else {
                Some((x, x))
            }
        })
    }

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

#[derive(Debug, Error)]
pub enum SingleError {
    #[error("Iterator yielded no elements")] None,
    #[error("Iterator yielded more than one element")] More
}