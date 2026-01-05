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

    const INPUT: &str = include_str!("../test_data/day_01.txt");

    #[test_case(EXAMPLE_1 => 3)]
    #[test_case("+1\n+1\n+1" => 3)]
    #[test_case("+1\n+1\n-2" => 0)]
    #[test_case("-1\n-2\n-3" => -6)]
    #[test_case(INPUT => 599)]
    fn part_1(input: &str) -> i64 {
        super::part_1(input)
    }

    #[test_case(EXAMPLE_1 => 2)]
    #[test_case("+1\n-1" => 0)]
    #[test_case("+3\n+3\n+4\n-2\n-4" => 10)]
    #[test_case("-6\n+3\n+8\n+5\n-6" => 5)]
    #[test_case("+7\n+7\n-2\n-7\n-4" => 14)]
    #[test_case(INPUT => 81204)]
    fn part_2(input: &str) -> i64 {
        super::part_2(input)
    }
}
