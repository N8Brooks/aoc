use itertools::Itertools;

pub fn part_1(input: &str) -> i64 {
    input.lines().map(|line| line.parse::<i64>().unwrap()).sum()
}

pub fn part_2(input: &str) -> i64 {
    Some(0)
        .into_iter()
        .chain(input.lines().map(|line| line.parse().unwrap()).cycle())
        .scan(0, |sum, num| {
            *sum += num;
            Some(*sum)
        })
        .duplicates()
        .next()
        .unwrap_or_else(|| unreachable!())
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE_1: &str = "+1\n-2\n+3\n+1";

    const INPUT: &str = include_str!("../../../test_data/year_2018/day_01.txt");

    #[test_case(EXAMPLE_1, 3; "example_1")]
    #[test_case("+1\n+1\n+1", 3; "example_2")]
    #[test_case("+1\n+1\n-2", 0; "example_3")]
    #[test_case("-1\n-2\n-3", -6; "example_4")]
    #[test_case(INPUT, 599; "input")]
    fn part_1(input: &str, expected: i64) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE_1, 2; "example_1")]
    #[test_case("+1\n-1", 0; "example_2")]
    #[test_case("+3\n+3\n+4\n-2\n-4", 10; "example_3")]
    #[test_case("-6\n+3\n+8\n+5\n-6", 5; "example_4")]
    #[test_case("+7\n+7\n-2\n-7\n-4", 14; "example_5")]
    #[test_case(INPUT, 81204; "input")]
    fn part_2(input: &str, expected: i64) {
        assert_eq!(super::part_2(input), expected);
    }
}
