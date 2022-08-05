use num::Integer;
use std::{iter::Product, ops::AddAssign};

pub fn egcd<T>(a: T, b: T) -> (T, T, T)
where
    T: Copy + Integer,
{
    if a == T::zero() {
        (b, T::zero(), T::one())
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

pub fn mod_inv<T>(x: T, n: T) -> Option<T>
where
    T: Copy + Integer,
{
    let (g, x, _) = egcd(x, n);
    if g == T::one() {
        Some((x % n + n) % n)
    } else {
        None
    }
}

pub fn crt<'a, T>(residues: &[T], modulii: &'a [T]) -> Option<T>
where
    T: AddAssign + Copy + Integer + Product<&'a T>,
{
    let prod = modulii.iter().product::<T>();
    let mut sum = T::zero();
    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }
    Some(sum % prod)
}
