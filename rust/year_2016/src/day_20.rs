use std::error::Error;
use std::fmt::Display;
use std::num::ParseIntError;

use itertools::Itertools as _;

pub fn part_1(input: &str) -> Result<Option<u32>, ParseRangeError> {
    let mut min = u32::MIN;
    for (start, end) in parse_ranges(input)? {
        if start > min {
            break;
        }
        let Some(nxt) = end.checked_add(1) else {
            return Ok(None);
        };
        min = min.max(nxt);
    }
    Ok(Some(min))
}

pub fn part_2(input: &str) -> Result<u32, ParseRangeError> {
    let mut count = 0;
    let mut min = u32::MIN;
    for (start, end) in parse_ranges(input)? {
        if let Some(diff) = start.checked_sub(min) {
            count += diff;
        }
        let Some(nxt) = end.checked_add(1) else {
            break;
        };
        min = min.max(nxt);
    }
    Ok(count)
}

fn parse_ranges(input: &str) -> Result<Vec<(u32, u32)>, ParseRangeError> {
    input
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').ok_or(ParseRangeError::MissingDash)?;
            let start = start.parse().map_err(ParseRangeError::ParseStart)?;
            let end = end.parse().map_err(ParseRangeError::ParseEnd)?;
            Ok((start, end))
        })
        .try_collect()
        .map(|mut ranges: Vec<_>| {
            ranges.sort_unstable_by_key(|range| range.0);
            ranges
        })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseRangeError {
    MissingDash,
    ParseStart(ParseIntError),
    ParseEnd(ParseIntError),
}

impl Display for ParseRangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingDash => "missing dash in range".fmt(f),
            Self::ParseStart(err) | Self::ParseEnd(err) => err.fmt(f),
        }
    }
}

impl Error for ParseRangeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::MissingDash => None,
            Self::ParseStart(err) | Self::ParseEnd(err) => Some(err),
        }
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_20.txt");

    const EXAMPLE: &str = "5-8
0-2
4-7";

    #[test_case(EXAMPLE => Ok(Some(3)))]
    #[test_case(INPUT => Ok(Some(31053880)))]
    fn part_1(input: &str) -> Result<Option<u32>, super::ParseRangeError> {
        super::part_1(input)
    }

    #[test_case(INPUT => Ok(117))]
    fn part_2(input: &str) -> Result<u32, super::ParseRangeError> {
        super::part_2(input)
    }
}
