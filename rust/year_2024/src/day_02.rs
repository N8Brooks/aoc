use itertools::Itertools;

pub fn part_1(input: &str) -> usize {
    let reports = parse_input(input);
    reports
        .into_iter()
        .filter(|report| is_valid(report))
        .count()
}

pub fn part_2(input: &str) -> usize {
    let reports = parse_input(input);
    reports
        .filter(|report| {
            // n^2 could be n, but short column sizes
            report.iter().enumerate().any(|(i, _)| {
                let report = report
                    .iter()
                    .enumerate()
                    .filter_map(|(j, num)| (i != j).then_some(num));
                is_valid(report)
            })
        })
        .count()
}

fn is_valid<'a>(report: impl IntoIterator<Item = &'a usize>) -> bool {
    let deltas = report
        .into_iter()
        .map(|&num| num as isize)
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec();
    deltas.iter().all(|d| (-3..=-1).contains(d)) || deltas.iter().all(|d| (1..=3).contains(d))
}

fn parse_input(input: &str) -> impl Iterator<Item = Vec<usize>> + '_ {
    input.lines().map(|line| {
        line.split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect()
    })
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../../../testdata/year_2024/day_02.txt");

    const EXAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test_case(EXAMPLE, 2)]
    #[test_case(INPUT, 486)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 4)]
    #[test_case(INPUT, 540)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
