use std::{cmp::{max, min}, ops::{Add, Sub}};

use itertools::{IntoChunks, Itertools};
use num_traits::Zero;

use super::Point;

/// Represents an area at a location
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Area<T = usize> {
    pub position: Point<T>,
    pub dimensions: (usize, usize)
}

impl<T> Area<T> {
    /// Creates a new area at `position` with dimensions `dimensions`
    #[must_use]
    pub const fn new(position: Point<T>, dimensions: (usize, usize)) -> Self {
        Self { position, dimensions }
    }

    /// Creates a new area at `position` with dimensions `dimensions`
    #[must_use]
    pub fn from_dimensions(width: usize, height: usize) -> Self where
        T: Zero
    {
        Self {
            position: Point::zero(),
            dimensions: (width, height)
        }
    }

    /// Computes the surface area of the area
    pub fn surface_area(self) -> usize {
        let (width, height) = self.dimensions;
        width * height
    }

    /// Determines if `point` is contained in the area
    pub fn contains<U>(self, point: Point<U>) -> bool where
        T: Copy + PartialOrd + Add<Output=T> + TryFrom<usize>,
        U: TryInto<T>
    {
        let bottom_right = self.position + Point::from(self.dimensions).cast::<T>().unwrap();
        let Some(point) = point.cast::<T>() else { return false; };

        point.x >= self.position.x
            && point.y >= self.position.y
            && point.x < bottom_right.x
            && point.y < bottom_right.y
    }

    /// Computes the minimal bounding area around a set of points
    pub fn bounding_area<I>(points: I) -> Self where
        T: Ord + Zero + Sub<Output=T> + TryInto<usize> + Copy,
        I: IntoIterator<Item=Point<T>>
    {
        let Some((top_left, bottom_right)) = points
            .into_iter()
            .fold(None, |bounds, point| {
                Some(bounds.map_or((point, point), |(low, high): (Point<T>, Point<T>)| (
                    Point { x: min(low.x, point.x), y: min(low.y, point.y) },
                    Point { x: max(high.x, point.x), y: max(high.y, point.y) }
                )))
            }) else { return Self::from_dimensions(0, 0) };


        let dimensions = (bottom_right - top_left)
            .cast::<usize>()
            .unwrap() + Point::one();

        Self::new(top_left, dimensions.into())
    }

    /// Iterate over the points contained in the area.
    /// The points are visited left-to-right, top-to-bottom
    pub fn iter(&self) -> Iter<T> where
        T: TryFrom<usize> + Add<Output=T> + Copy
    {
        self.into_iter()
    }

    /// Iterate over the points contained in the area row-by-row
    pub fn iter_rows(&self) -> IntoChunks<Iter<T>> where
        T: TryFrom<usize> + Add<Output=T> + Copy
    {
        self
            .iter()
            .chunks(self.dimensions.0)
    }
}

impl<T> From<(usize, usize)> for Area<T> where
    T: Zero
{
    fn from((width, height): (usize, usize)) -> Self {
        Self::from_dimensions(width, height)
    }
}

impl<T> IntoIterator for Area<T> where
    T: TryFrom<usize> + Add<Output=T> + Copy
{
    type Item = Point<T>;
    type IntoIter = Iter<T>;
    
    /// Iterate over the points contained in the area.
    /// The points are visited left-to-right, top-to-bottom
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            area: self,
            index: 0,
            end: self.surface_area()
        }    
    }
}

impl<T> IntoIterator for &Area<T> where
    T: TryFrom<usize> + Add<Output=T> + Copy
{
    type Item = Point<T>;
    type IntoIter = Iter<T>;
    
    /// Iterate over the points contained in the area.
    /// The points are visited left-to-right, top-to-bottom
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            area: *self,
            index: 0,
            end: self.surface_area()
        }    
    }
}

/// Iterates over all the [`Point`]s in an [`Area`]
/// 
/// The iterations happens left-to-right, top-to-bottom
pub struct Iter<T> {
    area: Area<T>,
    index: usize,
    end: usize
}

impl<T> Iterator for Iter<T> where
    T: TryFrom<usize> + Add<Output=T> + Copy
{
    type Item = Point<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.end { return None; }

        let width= self.area.dimensions.0;
        let offset = Point {
            x: self.index % width,
            y: self.index / width
        }.cast::<T>().unwrap();

        self.index += 1;
        Some(self.area.position + offset)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.area.surface_area() - self.index;
        (size, Some(size))
    }
}

impl<T> DoubleEndedIterator for Iter<T> where
    T: TryFrom<usize> + Add<Output=T> + Copy
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.end <= self.index { return None; }
        self.end -= 1;

        let width = self.area.dimensions.0;
        let offset = Point {
            x: self.end % width,
            y: self.end / width
        }.cast::<T>().unwrap();

        Some(self.area.position + offset)
    }
}

impl<T> ExactSizeIterator for Iter<T> where
    T: TryFrom<usize> + Add<Output=T> + Copy
{}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;

    use super::*;

    #[test]
    fn area_surface_area() {
        assert_eq!(12, Area {
            position: Point { x: -3, y: 0 },
            dimensions: (4, 3)
        }.surface_area());
    }

    #[test]
    fn area_from_dimensions() {
        assert_eq!(Area {
            position: Point::<usize>::zero(),
            dimensions: (3, 3)
        }, Area::from_dimensions(3, 3));
    }
    
    #[test]
    fn area_contains() {
        assert!(Area::<usize>::from_dimensions(2, 2).contains(Point::<isize>::one()));
        assert!(!Area::<usize>::from_dimensions(0, 0).contains(Point::<usize>::zero()));
        assert!(!Area::<usize>::from_dimensions(2, 2).contains(Point::new(-1, -1)));
    }

    #[test]
    fn area_into_iter() {
        let area = Area::from_dimensions(2, 3);
        let points = [(0, 0), (1, 0), (0, 1), (1, 1), (0, 2), (1, 2)].map(Point::from);

        assert_eq!(points.len(), area.into_iter().len());
        assert_equal(points, area);
        assert_equal(
            points.into_iter().rev(),
            area.into_iter().rev()
        );

        let area = Area::new(Point::one(), (2, 2));
        let points = [(1, 1), (2, 1), (1, 2), (2, 2)].map(Point::from);

        assert_eq!(points.len(), area.into_iter().len());
        assert_equal(points, area);
        assert_equal(
            points.into_iter().rev(),
            area.into_iter().rev()
        );
    }

    #[test]
    fn area_iter_rows() {
        assert_equal(
            [
                vec![Point::new(1, 1), Point::new(2, 1)],
                vec![Point::new(1, 2), Point::new(2, 2)]
            ],
            Area { position: Point::one(), dimensions: (2, 2) }
                .iter_rows()
                .into_iter()
                .map(Vec::from_iter)
        );
    }

    #[test]
    fn area_bounding_area() {
        assert_eq!(
            Area { position: Point::new(2, 1), dimensions: (2, 4) },
            Area::bounding_area([(2, 2), (3, 1), (3, 4), (2, 1)].map(Point::from))
        );

        assert_eq!(Area::<usize>::from_dimensions(0, 0), Area::bounding_area([]));
    }
}