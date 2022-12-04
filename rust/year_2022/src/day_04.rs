use std::ops::RangeInclusive;

pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let (a, b) = line.split_once(',').unwrap();
            let a = get_range(a);
            let b = get_range(b);
            a.contains(b.start()) && a.contains(b.end())
                || b.contains(a.start()) && b.contains(a.end())
        })
        .count()
}

pub fn part_2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let (a, b) = line.split_once(',').unwrap();
            let a = get_range(a);
            let b = get_range(b);
            a.contains(b.start())
                || a.contains(b.end())
                || b.contains(a.start())
                || b.contains(a.end())
        })
        .count()
}

fn get_range(range: &str) -> RangeInclusive<usize> {
    let (start, stop) = range.split_once('-').unwrap();
    let start = start.parse().unwrap();
    let stop = stop.parse().unwrap();
    start..=stop
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    const INPUT: &str = include_str!("../../../testdata/year_2022/day_04.txt");

    #[test_case(EXAMPLE, 2)]
    #[test_case(INPUT, 540)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 4)]
    #[test_case(INPUT, 872)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
