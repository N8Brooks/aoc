use std::{error::Error, fmt, iter::successors};

pub fn part_1(input: &str) -> Result<u32, ParseMapError> {
    count_safe_tiles(input, 40)
}

pub fn part_2(input: &str) -> Result<u32, ParseMapError> {
    count_safe_tiles(input, 400000)
}

fn count_safe_tiles(input: &str, n: usize) -> Result<u32, ParseMapError> {
    let tiles = parse_map(input)?;
    let m = input.len().try_into().unwrap();
    let mask = 1u128.checked_shl(m).map_or(u128::MAX, |s| s - 1);
    let count = successors(Some(tiles), |tiles| Some((tiles << 1 ^ tiles >> 1) & mask))
        .take(n)
        .map(|tiles| m - tiles.count_ones())
        .sum();
    Ok(count)
}

fn parse_map(input: &str) -> Result<u128, ParseMapError> {
    if input.len() > u128::BITS as usize {
        return Err(ParseMapError::TooLong);
    }
    input
        .bytes()
        .map(|c| match c {
            b'.' => Ok(0),
            b'^' => Ok(1),
            _ => Err(ParseMapError::InvalidChar),
        })
        .try_fold(0, |acc, bit| Ok((acc << 1) | bit?))
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub enum ParseMapError {
    InvalidChar,
    TooLong,
}

impl fmt::Display for ParseMapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidChar => "invalid character in map",
            Self::TooLong => "map is too long to fit in u128",
        }
        .fmt(f)
    }
}

impl Error for ParseMapError {}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_18.txt");

    #[test_case(INPUT => Ok(1939))]
    fn part_1(input: &str) -> Result<u32, super::ParseMapError> {
        super::part_1(input)
    }

    #[test_case(INPUT => Ok(19999535))]
    fn part_2(input: &str) -> Result<u32, super::ParseMapError> {
        super::part_2(input)
    }

    #[test_case("..^^.", 3 => Ok(6))]
    #[test_case(".^^.^.^^^^", 10 => Ok(38))]
    fn count_safe_tiles(input: &str, n: usize) -> Result<u32, super::ParseMapError> {
        super::count_safe_tiles(input, n)
    }
}
