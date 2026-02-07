use std::iter::{self};

use itertools::Itertools as _;

pub fn part_1(input: &str) -> String {
    let start: [u8; 8] = input.as_bytes().try_into().unwrap();
    let password = passwords(start).find(valid).unwrap();
    String::from_utf8(password.into()).unwrap()
}

pub fn part_2(input: &str) -> String {
    let start: [u8; 8] = input.as_bytes().try_into().unwrap();
    let password = passwords(start).filter(valid).nth(1).unwrap();
    String::from_utf8(password.into()).unwrap()
}

fn passwords<const N: usize>(start: [u8; N]) -> impl Iterator<Item = [u8; N]> {
    const POOL: [u8; 23] = *b"abcdefghjkmnpqrstuvwxyz";
    let mut indexes = start.map(|b| POOL.iter().position(|a| a == &b).unwrap());
    iter::from_fn(move || {
        indexes
            .iter_mut()
            .rev()
            .any(|i| {
                let inc = *i < POOL.len() - 1;
                let mask = (inc as usize).wrapping_neg();
                *i = (*i + 1) & mask;
                inc
            })
            .then(|| indexes.map(|i| POOL[i]))
    })
}

#[inline(always)]
fn valid<const N: usize>(indexes: &[u8; N]) -> bool {
    indexes
        .array_windows()
        .any(|[a, b, c]| a + 1 == *b && b + 1 == *c)
        && indexes
            .array_windows()
            .filter(|[a, b]| a == b)
            .map(|w| w[0])
            .tuple_windows()
            .any(|(a, b)| a != b)
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_11.txt");

    #[test_case(INPUT => "hxbxxyzz")]
    fn part_1(input: &str) -> String {
        super::part_1(input)
    }

    #[test_case(INPUT => "hxcaabcc")]
    fn part_2(input: &str) -> String {
        super::part_2(input)
    }
}
