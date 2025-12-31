use std::{array, ops::Rem};

use num::{BigInt, One as _, Zero, traits::Euclid};

enum Technique {
    DealIntoNewStack,
    Cut(isize),
    DealWithIncrement(usize),
}

impl From<&str> for Technique {
    fn from(line: &str) -> Self {
        if line == "deal into new stack" {
            Technique::DealIntoNewStack
        } else if let Some(cut_str) = line.strip_prefix("cut ") {
            let n = cut_str.parse().unwrap();
            Technique::Cut(n)
        } else if let Some(inc_str) = line.strip_prefix("deal with increment ") {
            let n = inc_str.parse().unwrap();
            Technique::DealWithIncrement(n)
        } else {
            panic!("Invalid technique: {}", line)
        }
    }
}

pub fn part_1(input: &str) -> usize {
    const N: usize = 10_007;
    let deck = array::from_fn(|i| i);
    input
        .lines()
        .map(Technique::from)
        .fold(deck, |mut deck, tech| {
            match tech {
                Technique::DealIntoNewStack => {
                    deck.reverse();
                }
                Technique::Cut(k) => {
                    let k = k.rem_euclid(N.try_into().unwrap());
                    deck.rotate_left(k.try_into().unwrap());
                }
                Technique::DealWithIncrement(inc) => {
                    let mut new_deck = [0; N];
                    for (i, &card) in deck.iter().enumerate() {
                        new_deck[(i * inc) % N] = card;
                    }
                    deck = new_deck;
                }
            }
            deck
        })
        .into_iter()
        .position(|card| card == 2019)
        .unwrap()
}

pub fn part_2(input: &str) -> usize {
    // Size of the deck
    let n = BigInt::from(119_315_717_514_047i64);
    // Number of shuffles
    let m = BigInt::from(101_741_582_076_661i64);
    // Position to query
    let x = BigInt::from(2020);

    let (a, b) = input.lines().rev().map(Technique::from).fold(
        (BigInt::one(), BigInt::zero()),
        |(a, b), tech| match tech {
            Technique::DealIntoNewStack => ((&n - a) % &n, &n - b - 1),
            Technique::Cut(k) => (a, (b + BigInt::from(k)).rem_euclid(&n)),
            Technique::DealWithIncrement(inc) => {
                let inv = &BigInt::from(inc).modinv(&n).unwrap();
                (a * inv % &n, b * inv % &n)
            }
        },
    );

    let a_pow = a.modpow(&m, &n);
    let b_pow = if a == BigInt::one() {
        (b * m) % &n
    } else {
        let numer = (BigInt::one() - &a_pow).rem_euclid(&n);
        let denom = (BigInt::one() - &a).rem_euclid(&n);
        let denom_inv = denom.modinv(&n).unwrap();
        b * numer % &n * denom_inv % &n
    };
    (a_pow * x + b_pow).rem(&n).try_into().unwrap()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_22.txt");

    #[test_case(INPUT => 3377)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(INPUT => 29988879027217)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
