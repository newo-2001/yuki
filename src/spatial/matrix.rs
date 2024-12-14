use std::ops::{Index, IndexMut};

use nom::{multi::many1, Parser, combinator::map_res};
use thiserror::Error;
use itertools::Itertools;

use crate::{iterators::{Enumerate2D, ExtraIter, TryFromIterator}, parsing::{combinators::lines, Parsable, ParsingResult}};

use super::Point;

/// A Matrix is a dense `N * M` 2D array
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Matrix<T> {
    data: Box<[T]>,
    columns: usize
}

/// Error returned when attempting to create a [`Matrix`] with variable row sizes
#[derive(Debug, Error, Clone, Copy)]
#[error("Cannot construct a matrix from variable rows")]
pub struct VariableRows;

impl<T, I> TryFromIterator<I> for Matrix<T> where
    I: Iterator,
    I::Item: IntoIterator<Item=T>,
    <I::Item as IntoIterator>::IntoIter: Clone
{
    type Item = I::Item;
    type Error = VariableRows;

    fn try_from_iter(iter: I) -> Result<Self, Self::Error> {
        let mut columns: Option<usize> = None;

        let data = iter
            .into_iter()
            .map(|row| {
                let row = row.into_iter();
                let length = row.clone().count();
                match columns {
                    Some(expected) if expected != length => Err(VariableRows),
                    Some(_) => Ok(row),
                    None => {
                        columns = Some(length);
                        Ok(row)
                    }
                }
            })
            .flatten_ok()
            .collect::<Result<Vec<_>, _>>()?
            .into_boxed_slice();

        Ok(Self {
            columns: columns.unwrap_or_default(),
            data
        })
    }
}

impl<T> Index<Point<usize>> for Matrix<T> {
    type Output = T;

    fn index(&self, index: Point<usize>) -> &Self::Output {
        let Point { x, y } = index;
        &self.data[y * self.columns + x]
    }
}

impl<T> IndexMut<Point<usize>> for Matrix<T> {
    fn index_mut(&mut self, index: Point<usize>) -> &mut Self::Output {
        let (row, col) = index.into();
        &mut self.data[row * self.columns + col]
    }
}

impl<T> IntoIterator for Matrix<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data
            .into_vec()
            .into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Matrix<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Matrix<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T> Matrix<T> {
    /// Returns the amount of columns the matrix has
    #[must_use]
    pub const fn cols(&self) -> usize {
        self.columns
    }

    #[must_use]
    /// Returns the amount of rows the matrix has
    pub const fn rows(&self) -> usize {
        match self.columns {
            0 => 0,
            columns => self.data.len() / columns
        }
    }

    #[must_use]
    /// Attempts to retrieve an element from the matrix at the specified index
    pub fn get(&self, index: Point<usize>) -> Option<&T> {
        let Point { x, y } = index;
        (x < self.cols() && y < self.rows())
            .then(|| self.index(index))
    }

    /// Creates an iterator over all the elements in the matrix
    /// 
    /// The iterator moves left-to-right, top-to-bottom
    pub fn iter(&self) -> core::slice::Iter<T> {
        self.data.iter()
    }
    
    /// Creates a mutable iterator over all the elements in the matrix
    /// 
    /// The iterator moves left-to-right, top-to-bottom
    pub fn iter_mut(&mut self) -> core::slice::IterMut<T> {
        self.data.iter_mut()
    }

    /// Create an iterator over all the rows in the matrix
    pub fn iter_rows(&self) -> core::slice::ChunksExact<T> {
        self.data.chunks_exact(self.columns)
    }

    /// Creates an iterator over all the columns in the matrix
    pub fn iter_cols(&self) -> impl Iterator<Item=impl ExactSizeIterator<Item=&T>> {
        (0..self.columns)
            .map(|col| {
                self.data
                    .iter()
                    .skip(col)
                    .step_by(self.columns)
            })
    }

    /// Creates a consuming iterator that drains the matrix row by row
    #[must_use]
    pub fn into_rows(self) -> IntoRows<T> {
        IntoRows {
            data: self.data.into_vec(),
            columns: self.columns
        }
    }

    /// Creates a consuming iterator that drains the matrix column by column
    #[must_use]
    pub fn into_cols(self) -> IntoRows<T> where T: Clone {
        self
            .transpose()
            .into_rows()
    }

    /// Transposes the matrix, switching rows and columns
    #[must_use]
    pub fn transpose(self) -> Self where T: Clone {
        let columns = self.rows();
        let data: Box<[T]> = self
            .iter_cols()
            .flatten()
            .cloned()
            .collect();

        Self { data, columns }
    }

    /// Perform a mapping on every element of the matrix
    /// using the specified mapping function
    #[must_use]
    pub fn map<F, U>(&self, mapper: F) -> Matrix<U> where
        F: Fn((Point<usize>, &T)) -> U
    {
        let data: Box<[U]> = self
            .iter_rows()
            .enumerate2d()
            .map(mapper)
            .collect();

        Matrix {
            columns: self.columns,
            data
        }
    }
}

impl<'a, T> Parsable<'a> for Matrix<T> where
    T: Parsable<'a> + Clone
{
    fn parse(input: &'a str) -> ParsingResult<'a, Self> {
        map_res(
            lines(
                many1(T::parse)
            ),
            |matrix| matrix.into_iter().try_collecting()
        )
        .parse(input)
    }
}

/// An iterator that drains a matrix row by row
pub struct IntoRows<T> {
    data: Vec<T>,
    columns: usize
}

impl<T> Iterator for IntoRows<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() { return None }
        
        let mut chunk = self.data.split_off(self.columns);
        std::mem::swap(&mut chunk, &mut self.data);

        Some(chunk)
    }
}