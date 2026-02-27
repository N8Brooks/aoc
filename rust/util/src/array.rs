/// Transposes a 2D array of size MxN into one of size NxM.
pub fn transpose<const M: usize, const N: usize, T>(m: [[T; N]; M]) -> [[T; M]; N] {
    use std::array::from_fn;
    let mut iters = m.map(|r| r.into_iter());
    from_fn(|_| from_fn(|i| iters[i].next().unwrap()))
}
