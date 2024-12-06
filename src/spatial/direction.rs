use nom::{branch::alt, character::complete::{char, one_of}, combinator::value, Parser};
use num_traits::Signed;

use crate::parsing::{Parsable, ParsingResult};

pub trait Directions: Sized {
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

impl Cardinal {
    #[must_use]
    pub const fn turn(self, direction: Rotation) -> Self {
        use Rotation::{Clockwise as CW, CounterClockwise as CCW};

        match (direction, self) {
            (CW, Self::North) | (CCW, Self::South) => Self::East,
            (CW, Self::East) | (CCW, Self::West)=> Self::South,
            (CW, Self::South) | (CCW, Self::North)=> Self::West,
            (CW, Self::West) | (CCW, Self::East) => Self::North
        }
    }
}

impl<'a> Parsable<'a> for Cardinal {
    fn parse(input: &'a str) -> ParsingResult<'a, Self> {
        alt((
            value(Self::North, char('^')),
            value(Self::East, char('>')),
            value(Self::South, one_of("Vv")),
            value(Self::West, char('<'))
        ))
        .parse(input)
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

impl Ordinal {
    #[must_use]
    pub const fn turn(self, direction: Rotation) -> Self {
        use Rotation::{Clockwise as CW, CounterClockwise as CCW};

        match (direction, self) {
            (CW, Self::NorthEast) | (CCW, Self::SouthWest) => Self::SouthEast,
            (CW, Self::SouthEast) | (CCW, Self::NorthWest) => Self::SouthWest,
            (CW, Self::SouthWest) | (CCW, Self::NorthEast) => Self::NorthWest,
            (CW, Self::NorthWest) | (CCW, Self::SouthEast) => Self::NorthEast
        }
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

impl Compass {
    #[must_use]
    pub const fn turn(self, direction: Rotation) -> Self {
        use Rotation::{Clockwise as CW, CounterClockwise as CCW};
        use self::{Cardinal as Card, Ordinal as Ord};

        match (direction, self) {
            (CW, Self::Cardinal(Card::North)) | (CCW, Self::Cardinal(Card::East)) => Self::Ordinal(Ord::NorthEast),
            (CW, Self::Cardinal(Card::East)) | (CCW, Self::Cardinal(Card::South)) => Self::Ordinal(Ord::SouthEast),
            (CW, Self::Cardinal(Card::South)) | (CCW, Self::Cardinal(Card::West)) => Self::Ordinal(Ord::SouthWest),
            (CW, Self::Cardinal(Card::West)) | (CCW, Self::Cardinal(Card::North)) => Self::Ordinal(Ord::NorthWest),
            (CW, Self::Ordinal(Ord::NorthEast)) | (CCW, Self::Ordinal(Ord::SouthEast)) => Self::Cardinal(Card::East),
            (CW, Self::Ordinal(Ord::SouthEast)) | (CCW, Self::Ordinal(Ord::SouthWest)) => Self::Cardinal(Card::South),
            (CW, Self::Ordinal(Ord::SouthWest)) | (CCW, Self::Ordinal(Ord::NorthWest)) => Self::Cardinal(Card::West),
            (CW, Self::Ordinal(Ord::NorthWest)) | (CCW, Self::Ordinal(Ord::NorthEast)) => Self::Cardinal(Card::North)
        }
    }
}

impl<T: Signed> From<Compass> for (T, T) where {
    fn from(val: Compass) -> Self {
        val.vector()
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Rotation {
    Clockwise,
    CounterClockwise
}

impl Rotation {
    #[must_use]
    pub const fn inverted(self) -> Self {
        match self {
            Self::Clockwise => Self::CounterClockwise,
            Self::CounterClockwise => Self::Clockwise
        }
    }
}