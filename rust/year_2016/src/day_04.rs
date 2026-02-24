use std::cmp::Reverse;

use itertools::Itertools as _;

pub fn part_1(input: &str) -> u32 {
    parse_input(input)
        .filter(|(name, _, actual)| {
            let counts = name
                .bytes()
                .filter(|&b| b != b'-')
                .fold([0; 26], |mut counts, b| {
                    let i = (b - b'a') as usize;
                    counts[i] += 1;
                    counts
                });
            counts
                .into_iter()
                .enumerate()
                .k_smallest_by_key(5, |&(i, count)| (Reverse(count), i))
                .map(|(i, _)| i as u8 + b'a')
                .eq(actual.bytes())
        })
        .map(|(_, sector_id, _)| sector_id)
        .sum()
}

pub fn part_2(input: &str) -> u32 {
    parse_input(input)
        .find(|(name, sector_id, _)| {
            name.bytes()
                .map(|b| {
                    if b == b'-' {
                        b' '
                    } else {
                        let i = (b - b'a') as u32;
                        let j = (i + *sector_id) % 26;
                        j as u8 + b'a'
                    }
                })
                .eq(*b"northpole object storage")
        })
        .unwrap()
        .1
}

fn parse_input(input: &str) -> impl Iterator<Item = (&str, u32, &str)> {
    input.lines().map(|line| {
        let (name, rest) = line.rsplit_once('-').unwrap();
        let (sector_id, checksum) = rest.split_once('[').unwrap();
        let checksum = checksum.strip_suffix(']').unwrap();
        (name, sector_id.parse().unwrap(), checksum)
    })
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_04.txt");

    const EXAMPLE: &str = "aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]";

    #[test_case(EXAMPLE => 1514)]
    #[test_case(INPUT => 361724)]
    fn part_1(input: &str) -> u32 {
        super::part_1(input)
    }

    #[test_case(INPUT => 482)]
    fn part_2(input: &str) -> u32 {
        super::part_2(input)
    }
}
