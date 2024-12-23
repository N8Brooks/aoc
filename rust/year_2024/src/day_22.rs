use std::iter::once;

use hashbrown::HashMap;
use itertools::Itertools;

pub fn part_1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| (0..2000).fold(line.parse().unwrap(), |secret, _| next_secret(secret)))
        .sum()
}

pub fn part_2(input: &str) -> i64 {
    const N: i64 = 19;
    const M: i64 = N.pow(4);
    let mut total = HashMap::new();
    let mut bananas = HashMap::new();
    for line in input.lines() {
        let secret = line.parse().unwrap();
        once(secret)
            .chain((0..2000).scan(secret, |secret, _| {
                *secret = next_secret(*secret);
                Some(*secret)
            }))
            .map(|secret| secret % 10)
            .tuple_windows()
            .map(|(a, b)| (b - a, b))
            .scan(0, |changes, (change, price)| {
                *changes = (*changes * N + change + 9) % M;
                Some((*changes, price))
            })
            .skip(3)
            .for_each(|(changes, price)| {
                bananas.entry(changes).or_insert(price);
            });
        for (changes, price) in bananas.drain() {
            *total.entry(changes).or_default() += price;
        }
    }
    total.into_values().max().unwrap()
}

#[inline]
fn next_secret(mut secret: i64) -> i64 {
    const M: i64 = 16777216;
    secret ^= secret * 64;
    secret %= M;
    secret ^= secret / 32;
    secret %= M;
    secret ^= secret * 2048;
    secret %= M;
    secret
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_22.txt");

    const EXAMPLE_1: &str = "1
10
100
2024";

    #[test_case(EXAMPLE_1, 37327623)]
    #[test_case(INPUT, 13022553808)]
    fn part_1(input: &str, expected: i64) {
        assert_eq!(super::part_1(input), expected);
    }

    const EXAMPLE_2: &str = "1
2
3
2024";

    #[test_case(EXAMPLE_2, 23)]
    #[test_case(INPUT, 1555)]
    fn part_2(input: &str, expected: i64) {
        assert_eq!(super::part_2(input), expected);
    }
}
