use num_traits::Signed;

pub trait Directions {
    #[must_use]
    fn vector<T: Signed>(self) -> (T, T);

    #[must_use]
    fn inverted(self) -> Self;
    
    #[must_use]
    fn all() -> impl ExactSizeIterator<Item=Self>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cardinal {
    North,
    East,
    South,
    West
}

impl Directions for Cardinal {
    fn all() -> impl ExactSizeIterator<Item=Self> {
        [
            Self::North,
            Self::East,
            Self::South,
            Self::West
        ].into_iter()
    }

    fn inverted(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East
        }
    }

    fn vector<T: Signed>(self) -> (T, T) {
        match self {
            Self::North => (T::zero(), -T::one()),
            Self::East => (T::one(), T::zero()),
            Self::South => (T::zero(), T::one()),
            Self::West => (-T::one(), T::zero())
        }
    }
}

impl<T: Signed> From<Cardinal> for (T, T) where {
    fn from(val: Cardinal) -> Self {
        val.vector()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ordinal {
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest
}

impl Directions for Ordinal {
    fn vector<T: Signed>(self) -> (T, T) {
        match self {
            Self::NorthEast => (T::one(), -T::one()),
            Self::SouthEast => (T::one(), T::one()),
            Self::SouthWest => (-T::one(), T::one()),
            Self::NorthWest => (-T::one(), -T::one())
        }
    }
    
    fn inverted(self) -> Self {
        match self {
            Self::NorthEast => Self::SouthWest,
            Self::SouthEast => Self::NorthWest,
            Self::SouthWest => Self::NorthEast,
            Self::NorthWest => Self::SouthEast
        }
    }

    fn all() -> impl ExactSizeIterator<Item=Self> {
        [
            Self::NorthEast,
            Self::SouthEast,
            Self::SouthWest,
            Self::NorthWest
        ].into_iter()
    }
}

impl<T: Signed> From<Ordinal> for (T, T) where {
    fn from(val: Ordinal) -> Self {
        val.vector()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Compass {
    Cardinal(Cardinal),
    Ordinal(Ordinal)
}

impl Directions for Compass {
    fn vector<T: Signed>(self) -> (T, T) {
        match self {
            Self::Cardinal(direction) => direction.vector(),
            Self::Ordinal(direction) => direction.vector()
        }
    }

    fn inverted(self) -> Self {
        match self {
            Self::Cardinal(direction) => Self::Cardinal(direction.inverted()),
            Self::Ordinal(direction) => Self::Ordinal(direction.inverted())
        }
    }

    fn all() -> impl ExactSizeIterator<Item=Self> {
        [
            Self::Cardinal(Cardinal::North),
            Self::Ordinal(Ordinal::NorthEast),
            Self::Cardinal(Cardinal::East),
            Self::Ordinal(Ordinal::SouthEast),
            Self::Cardinal(Cardinal::South),
            Self::Ordinal(Ordinal::SouthWest),
            Self::Cardinal(Cardinal::West),
            Self::Ordinal(Ordinal::NorthWest)
        ].into_iter()
    }
}

impl<T: Signed> From<Compass> for (T, T) where {
    fn from(val: Compass) -> Self {
        val.vector()
    }
}