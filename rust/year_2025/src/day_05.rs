use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    let (fresh, available) = input.split_once("\n\n").unwrap();
    let ranges: Vec<_> = fresh
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            let start: usize = start.parse().unwrap();
            let end: usize = end.parse().unwrap();
            start..=end
        })
        .collect();
    available
        .lines()
        .filter(|line| {
            let id = line.parse().unwrap();
            ranges.iter().any(|range| range.contains(&id))
        })
        .count()
}

pub fn part_2(input: &str) -> usize {
    let (fresh, _) = input.split_once("\n\n").unwrap();
    let mut cursor = 0;
    fresh
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            let start: usize = start.parse().unwrap();
            let end: usize = end.parse().unwrap();
            (start, end + 1)
        })
        .sorted_unstable_by_key(|&(start, _)| start)
        .map(|(start, end)| {
            let res = end.saturating_sub(start.max(cursor));
            cursor = cursor.max(end);
            res
        })
        .sum()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_05.txt");

    const EXAMPLE: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test_case(EXAMPLE => 3)]
    #[test_case(INPUT => 611)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 14)]
    #[test_case(INPUT => 345995423801866)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
