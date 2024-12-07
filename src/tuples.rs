/// Returns the first element of a tuple
pub fn fst<T, U>((fst, _): (T, U)) -> T { fst }

/// Returns the second element of a tuple
pub fn snd<T, U>((_, snd): (T, U)) -> U { snd }

/// Swaps the two elements of a tuple
pub fn swap<T, U>((fst, snd): (T, U)) -> (U, T) { (snd, fst) }

/// Trait for retrieving the first element of a tuple
pub trait Fst {
    type Output;

    /// Returns the first element of a tuple
    fn fst(self) -> Self::Output;
}

/// Trait for retrieving the second element of a tuple
pub trait Snd {
    type Output;

    // Returns the second element of a tuple
    fn snd(self) -> Self::Output;
}

/// Trait for swapping the two elements of a tuple
pub trait Swap {
    type Output;

    // Swaps the two elements of a tuple
    fn swap(self) -> Self::Output;
}

impl<T, U> Fst for (T, U) {
    type Output = T;
    
    fn fst(self) -> Self::Output {
        self.0
    }
}

impl<T, U> Snd for (T, U) {
    type Output = U;

    fn snd(self) -> Self::Output {
        self.1
    }
}

impl<T, U> Swap for (T, U) {
    type Output = (U, T);

    fn swap(self) -> Self::Output {
        (self.1, self.0)
    }
}