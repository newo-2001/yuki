use std::ops::Add;

use num_traits::Zero;

use super::Point;

/// Represents an area at a location
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Area<T = usize> {
    top_left: Point<T>,
    dimensions: (usize, usize)
}

impl<T> Area<T> {
    /// Creates a new area at `top_left` with dimensions `dimensions`
    #[must_use]
    pub const fn new(top_left: Point<T>, dimensions: (usize, usize)) -> Self {
        Self { top_left, dimensions }
    }

    /// Creates a new area at `top_left` with dimensions `dimensions`
    #[must_use]
    pub fn from_dimensions(width: usize, height: usize) -> Self where
        T: Zero
    {
        Self {
            top_left: Point::zero(),
            dimensions: (width, height)
        }
    }

    /// Retrieves the dimensions of the area
    pub fn dimensions(self) -> (usize, usize) {
        self.dimensions
    }

    /// Computes the surface area of the area
    pub fn surface_area(self) -> usize {
        let (width, height) = self.dimensions();
        width * height
    }

    /// Determines if `point` is contained in the area
    pub fn contains<U>(self, point: Point<U>) -> bool where
        T: Copy + PartialOrd + Add<Output=T> + TryFrom<usize>,
        U: TryInto<T>
    {
        let bottom_right = self.top_left + Point::from(self.dimensions).cast::<T>().unwrap();
        let Some(point) = point.cast::<T>() else { return false; };

        point.x >= self.top_left.x
            && point.y >= self.top_left.y
            && point.x < bottom_right.x
            && point.y < bottom_right.y
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
    
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            area: self,
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
        Some(self.area.top_left + offset)
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

        let width= self.area.dimensions.0;
        let offset = Point {
            x: self.end % width,
            y: self.end / width
        }.cast::<T>().unwrap();

        Some(self.area.top_left + offset)
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
    fn area_dimensions() {
        assert_eq!((2, 2), Area {
            top_left: Point { x: -4, y: -2 },
            dimensions: (2, 2)
        }.dimensions());
    }

    #[test]
    fn area_surface_area() {
        assert_eq!(12, Area {
            top_left: Point { x: -3, y: 0 },
            dimensions: (4, 3)
        }.surface_area());
    }

    #[test]
    fn area_from_dimensions() {
        assert_eq!(Area {
            top_left: Point::<usize>::zero(),
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
        let points = [(0, 0), (1, 0), (0, 1), (1, 1), (0, 2), (1, 2)]
            .map(Point::from);

        assert_eq!(points.len(), area.into_iter().len());
        assert_equal(points, area);
        assert_equal(
            points.into_iter().rev(),
            area.into_iter().rev()
        );
    }
}