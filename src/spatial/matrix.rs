use std::ops::{Index, IndexMut};

use thiserror::Error;

use crate::iterators::TryFromIterator;

use super::Point;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Matrix<T> {
    data: Box<[T]>,
    columns: usize
}

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

    fn try_from_iter(iter: I) -> Result<Self, Self::Error> where
    {
        let mut columns: Option<usize> = None;

        let data = iter
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
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
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
    #[must_use]
    pub const fn cols(&self) -> usize {
        self.columns
    }

    #[must_use]
    pub const fn rows(&self) -> usize {
        match self.columns {
            0 => 0,
            columns => self.data.len() / columns
        }
    }

    #[must_use]
    pub fn get(&self, index: Point<usize>) -> Option<&T> {
        let Point { x, y } = index;
        (x < self.cols() && y < self.rows())
            .then(|| self.index(index))
    }

    pub fn iter(&self) -> core::slice::Iter<T> {
        self.data.iter()
    }
    
    pub fn iter_mut(&mut self) -> core::slice::IterMut<T> {
        self.data.iter_mut()
    }

    pub fn iter_rows(&self) -> core::slice::ChunksExact<T> {
        self.data.chunks_exact(self.columns)
    }

    pub fn iter_cols(&self) -> impl Iterator<Item=impl ExactSizeIterator<Item=&T>> {
        (0..self.columns)
            .map(|col| {
                self.data
                    .iter()
                    .skip(col)
                    .step_by(self.columns)
            })
    }

    #[must_use]
    pub fn into_rows(self) -> IntoRows<T> {
        IntoRows {
            data: self.data.into_vec(),
            columns: self.columns
        }
    }

    #[must_use]
    pub fn into_cols(self) -> IntoRows<T> where T: Clone {
        self
            .transpose()
            .into_rows()
    }

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

    pub fn enumerate(&self) -> impl Iterator<Item=(Point<usize>, &T)>{
        self
            .iter_rows()
            .enumerate()
            .flat_map(|(y, row)| row
                .iter()
                .enumerate()
                .map(move |(x, value)| (Point { x, y }, value))
            )
    }

    #[must_use]
    pub fn map<F>(&self, mapper: F) -> Self where
        F: Fn((Point<usize>, &T)) -> T
    {
        let data: Box<[T]> = self
            .enumerate()
            .map(mapper)
            .collect();

        Self {
            columns: self.columns,
            data
        }
    }
}

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