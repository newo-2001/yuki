use std::ops::{Add, Div, Mul};

use num_traits::One;

/// Trait for the gaussian sum of contiguous ranges
pub trait GaussSum {
    type Output;

    /// Computes the guassian sum in `O(1)` time
    fn gauss_sum(self) -> Self::Output;
}

impl<T> GaussSum for T where
    T: Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> + One<Output = T>
{
    type Output = T;

    fn gauss_sum(self) -> Self::Output {
        let two = T::one() + T::one();
        self * (self + T::one()) / two
    }
}
