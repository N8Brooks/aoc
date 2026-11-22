use std::iter::successors;

use hashbrown::HashMap;
use num::{BigUint, ToPrimitive as _};

const N: usize = 20_201_227;
const M: usize = (N - 1).isqrt() + 1;

pub fn part_1(input: &str) -> usize {
    let (ec, ed) = input.trim().split_once('\n').unwrap();
    let ec = ec.parse().unwrap();
    let ed = ed.parse().unwrap();

    mod_pow(ec, baby_step_giant_step(ed), N)
}

fn baby_step_giant_step(base: usize) -> usize {
    let steps: HashMap<_, _> = successors(Some(1), |&value| Some(value * 7 % N))
        .take(M)
        .enumerate()
        .map(|(j, value)| (value, j))
        .collect();

    let factor = mod_pow(7, N - 1 - M, N);

    successors(Some(base), |&y| Some(y * factor % N))
        .take(M)
        .enumerate()
        .find_map(|(i, y)| steps.get(&y).map(|&j| i * M + j))
        .expect("discrete logarithm not found")
}

fn mod_pow(base: usize, exp: usize, modulus: usize) -> usize {
    BigUint::from(base)
        .modpow(&BigUint::from(exp), &BigUint::from(modulus))
        .to_usize()
        .unwrap()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_25.txt");

    const EXAMPLE: &str = "5764801
17807724";

    #[test_case(EXAMPLE => 14897079)]
    #[test_case(INPUT => 9177528)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }
}
