pub fn fst<T, U>((fst, _): (T, U)) -> T { fst }
pub fn snd<T, U>((_, snd): (T, U)) -> U { snd }