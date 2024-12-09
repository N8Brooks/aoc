use std::ops::RangeInclusive;

pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(get_ranges)
        .filter(|(a, b)| {
            let is_a_superset = a.contains(b.start()) && a.contains(b.end());
            let is_b_superset = b.contains(a.start()) && b.contains(a.end());
            is_a_superset || is_b_superset
        })
        .count()
}

pub fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(get_ranges)
        .filter(|(a, b)| {
            let is_a_in_b = a.contains(b.start()) || a.contains(b.end());
            let is_b_in_a = b.contains(a.start()) || b.contains(a.end());
            is_a_in_b || is_b_in_a
        })
        .count()
}

fn get_ranges(line: &str) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
    let get_range = |pair: &str| {
        let (start, end) = pair.split_once('-').unwrap();
        let start = start.parse().unwrap();
        let end = end.parse().unwrap();
        start..=end
    };
    let (a, b) = line.split_once(',').unwrap();
    (get_range(a), get_range(b))
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

    const INPUT: &str = include_str!("../../../test_data/year_2022/day_04.txt");

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
