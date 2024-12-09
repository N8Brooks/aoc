use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    let (lefts, rights) = parse_input(input);
    lefts
        .sorted_unstable()
        .zip(rights.sorted_unstable())
        .map(|(left, right)| left.abs_diff(right))
        .sum()
}

pub fn part_2(input: &str) -> usize {
    let (lefts, rights) = parse_input(input);
    let counts = rights.into_iter().counts();
    lefts
        .filter_map(|num| counts.get(&num).map(|count| num * count))
        .sum()
}

fn parse_input(
    input: &str,
) -> (
    impl Iterator<Item = usize> + '_,
    impl Iterator<Item = usize> + '_,
) {
    let (lefts, rights) = input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once("   ").unwrap();
            (left.parse().unwrap(), right.parse().unwrap())
        })
        .tee();
    (lefts.map(|(left, _)| left), rights.map(|(_, right)| right))
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../../../test_data/year_2024/day_01.txt");

    const EXAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test_case(EXAMPLE, 11)]
    #[test_case(INPUT, 3714264)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 31)]
    #[test_case(INPUT, 18805872)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
