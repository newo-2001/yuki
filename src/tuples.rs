pub fn fst<T, U>((fst, _): (T, U)) -> T { fst }
pub fn snd<T, U>((_, snd): (T, U)) -> U { snd }
pub fn swap<T, U>((fst, snd): (T, U)) -> (U, T) { (snd, fst) }

pub trait Fst {
    type Output;

    fn fst(self) -> Self::Output;
}

pub trait Snd {
    type Output;

    fn snd(self) -> Self::Output;
}

pub trait Swap {
    type Output;

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