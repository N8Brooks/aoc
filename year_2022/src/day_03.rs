use itertools::Itertools;
use std::collections::HashSet;

pub fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_at(line.len() / 2);
            let a: HashSet<u8> = HashSet::from_iter(a.bytes());
            let b: HashSet<u8> = HashSet::from_iter(b.bytes());
            let item = a.intersection(&b).next().unwrap();
            get_item_priority(item)
        })
        .sum()
}

pub fn part_2(input: &str) -> u32 {
    input
        .lines()
        .tuples()
        .map(|(a, b, c)| {
            let mut a: HashSet<u8> = HashSet::from_iter(a.bytes());
            let b: HashSet<u8> = HashSet::from_iter(b.bytes());
            let c: HashSet<u8> = HashSet::from_iter(c.bytes());
            a.retain(|char| b.contains(char) && c.contains(char));
            let item = a.iter().next().unwrap();
            get_item_priority(item)
        })
        .sum()
}

fn get_item_priority(item: &u8) -> u32 {
    if item.is_ascii_lowercase() {
        (item - b'a' + 1) as u32
    } else {
        (item - b'A' + 27) as u32
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    const INPUT: &str = include_str!("testdata/day_03.txt");

    #[test_case(EXAMPLE, 157)]
    #[test_case(INPUT, 8515)]
    fn part_1(input: &str, expected: u32) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 70)]
    #[test_case(INPUT, 2434)]
    fn part_2(input: &str, expected: u32) {
        assert_eq!(super::part_2(input), expected);
    }
}
