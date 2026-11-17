use std::num::NonZeroU32;

use itertools::Itertools as _;

pub fn part_1(input: &str) -> u32 {
    nth_spoken(input, 2020)
}

pub fn part_2(input: &str) -> u32 {
    nth_spoken(input, 30_000_000)
}

fn nth_spoken(input: &str, n: usize) -> u32 {
    let nums: Vec<usize> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut seen = vec![None; n];
    let mut indexes = NonZeroU32::new(1).unwrap()..NonZeroU32::new(n as u32).unwrap();

    for (&num, i) in nums.iter().dropping_back(1).zip(indexes.by_ref()) {
        seen[num] = Some(i);
    }

    let mut last = n
        .checked_sub(1)
        .and_then(|i| nums.get(i))
        .or_else(|| nums.last())
        .copied()
        .unwrap_or(0);
    for j in indexes {
        last = seen[last].replace(j).map_or(0, |i| j.get() - i.get()) as usize;
    }
    last as u32
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_15.txt");

    #[test_case("0,3,6" => 436)]
    #[test_case("1,3,2" => 1)]
    #[test_case("2,1,3" => 10)]
    #[test_case("1,2,3" => 27)]
    #[test_case("2,3,1" => 78)]
    #[test_case("3,2,1" => 438)]
    #[test_case("3,1,2" => 1836)]
    #[test_case(INPUT => 1111)]
    fn part_1(input: &str) -> u32 {
        super::part_1(input)
    }

    #[test_case("0,3,6" => 175594)]
    #[test_case("1,3,2" => 2578)]
    #[test_case("2,1,3" => 3544142)]
    #[test_case("1,2,3" => 261214)]
    #[test_case("2,3,1" => 6895259)]
    #[test_case("3,2,1" => 18)]
    #[test_case("3,1,2" => 362)]
    #[test_case(INPUT => 48568)]
    fn part_2(input: &str) -> u32 {
        super::part_2(input)
    }
}
