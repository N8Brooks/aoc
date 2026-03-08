use std::error::Error;

use itertools::Itertools;
use num::Integer;

pub fn part_1(input: &str) -> Result<String, ParseInitialStateError> {
    checksum(input, 272)
}

pub fn part_2(input: &str) -> Result<String, ParseInitialStateError> {
    checksum(input, 35651584)
}

pub fn checksum(input: &str, n: usize) -> Result<String, ParseInitialStateError> {
    // let mut a = parse_initial_state(input)?;
    // // Modified dragon curve
    // while a.len() < n {
    //     let b: Vec<bool> = a.iter().rev().map(|x| !x).collect();
    //     a.push(false);
    //     a.extend(b);
    // }
    // // Take first `n` bits
    // a.truncate(n);
    // // Checksum
    // while {
    //     a = a.as_chunks().0.iter().map(|[x, y]| x == y).collect();
    //     a.len().is_even()
    // } {}
    // Ok(a.into_iter().map(|b| if b { '1' } else { '0' }).collect())

    let initial_state = parse_initial_state(input)?;
    let m = initial_state.len();
    let data = (0..n).map(|i| {
        if (i + 1).is_multiple_of(m + 1) {
            let x = i / (m + 1) + 1;
            let k = x.trailing_zeros();
            (x >> (k + 1)).is_odd()
        } else if (i / (m + 1)).is_even() {
            initial_state[i % (m + 1)]
        } else {
            !initial_state[initial_state.len() - 1 - (i % (m + 1))]
        }
    });
    let checksum = data
        .chunks(1 << n.trailing_zeros())
        .into_iter()
        .map(|x| {
            if x.into_iter().filter(|&b| b).count().is_even() {
                '1'
            } else {
                '0'
            }
        })
        .collect();
    Ok(checksum)
}

fn parse_initial_state(input: &str) -> Result<Vec<bool>, ParseInitialStateError> {
    input
        .trim()
        .chars()
        .map(|c| match c {
            '0' => Ok(false),
            '1' => Ok(true),
            _ => Err(InitialStateErrorKind::InvalidDigit.into()),
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub struct ParseInitialStateError {
    kind: InitialStateErrorKind,
}

impl std::fmt::Display for ParseInitialStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            InitialStateErrorKind::InvalidDigit => "invalid digit in initial state",
        }
        .fmt(f)
    }
}

impl Error for ParseInitialStateError {}

impl From<InitialStateErrorKind> for ParseInitialStateError {
    fn from(kind: InitialStateErrorKind) -> Self {
        Self { kind }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
#[non_exhaustive]
enum InitialStateErrorKind {
    InvalidDigit,
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_16.txt");

    const EXAMPLE: &str = "10000";

    #[test_case(EXAMPLE, 20 => Ok("01100".into()))]
    fn checksum(input: &str, n: usize) -> Result<String, super::ParseInitialStateError> {
        super::checksum(input, n)
    }

    #[test_case(INPUT => Ok("10100011010101011".into()))]
    fn part_1(input: &str) -> Result<String, super::ParseInitialStateError> {
        super::part_1(input)
    }

    #[test_case(INPUT => Ok("01010001101011001".into()))]
    fn part_2(input: &str) -> Result<String, super::ParseInitialStateError> {
        super::part_2(input)
    }
}
