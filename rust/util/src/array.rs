pub trait Transpose {
    type Output;
    fn transpose(self) -> Self::Output;
}

impl<const M: usize, const N: usize, T> Transpose for [[T; N]; M] {
    type Output = [[T; M]; N];

    /// Transposes a 2D array of size MxN into one of size NxM.
    fn transpose(self) -> Self::Output {
        use std::array::from_fn;
        let mut iters = self.map(|r| r.into_iter());
        from_fn(|_| from_fn(|i| iters[i].next().unwrap()))
    }
}
