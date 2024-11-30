use std::ops::{Add, Div, Mul};

use num_traits::One;

pub trait GaussSum {
    type Output;

    fn gauss_sum(self) -> Self::Output;
}

impl<T> GaussSum for T where
    T: One + Add + Copy + Mul<<T as Add<T>>::Output>,
    <T as Mul<<T as Add<T>>::Output>>::Output: Div<<T as Add<T>>::Output>
{
    type Output = <<T as Mul<<T as Add>::Output>>::Output as Div<<T as Add<T>>::Output>>::Output;

    fn gauss_sum(self) -> Self::Output {
        let two = T::one() + T::one();
        self * (self + T::one()) / two
    }
}
