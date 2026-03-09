use std::error::Error;

use num::Integer;

pub fn part_1(input: &str) -> Result<String, ParseInitialStateError> {
    checksum(input, 272)
}

pub fn part_2(input: &str) -> Result<String, ParseInitialStateError> {
    checksum(input, 35651584)
}

pub fn checksum(input: &str, n: usize) -> Result<String, ParseInitialStateError> {
    let initial_state = parse_initial_state(input)?;
    let data = {
        let m = initial_state.len();
        let mut a = initial_state;
        let target = (n + 1).div_ceil(m + 1).next_power_of_two() * (m + 1) - 1;
        a.reserve_exact(target - a.len());
        let mut b = Vec::with_capacity(a.capacity() / 2);
        while a.len() < n {
            a.iter().rev().map(|x| !x).collect_into(&mut b);
            a.push(false);
            a.append(&mut b);
        }
        a.truncate(n); // take first `n` bits
        a
    };
    let checksum = data
        .chunks_exact(n & n.wrapping_neg()) // largest power of 2 dividing n
        .map(|chunk| chunk.iter().filter(|&&bit| bit).count().is_even())
        .map(|even_ones| if even_ones { '1' } else { '0' })
        .collect();
    Ok(checksum)
}

fn parse_initial_state(input: &str) -> Result<Vec<bool>, ParseInitialStateError> {
    input
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
