use itertools::Itertools as _;

pub fn part_1(input: &str) -> u32 {
    parse_input(input)
        .map(|row| {
            let (min, max) = row.minmax().into_option().unwrap();
            max - min
        })
        .sum()
}

pub fn part_2(input: &str) -> u32 {
    parse_input(input)
        .map(|row| {
            row.sorted_unstable()
                .array_combinations()
                .find(|[min, max]| max % min == 0)
                .map(|[min, max]| max / min)
                .unwrap()
        })
        .sum()
}

fn parse_input(input: &str) -> impl Iterator<Item = impl Iterator<Item = u32>> {
    input
        .lines()
        .map(|line| line.split_whitespace().map(|num| num.parse().unwrap()))
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_02.txt");

    const EXAMPLE_1: &str = "5 1 9 5
7 5 3
2 4 6 8";

    #[test_case(EXAMPLE_1 => 18)]
    #[test_case(INPUT => 51833)]
    fn part_1(input: &str) -> u32 {
        super::part_1(input)
    }

    const EXAMPLE_2: &str = "5 9 2 8
9 4 7 3
3 8 6 5";

    #[test_case(EXAMPLE_2 => 9)]
    #[test_case(INPUT => 288)]
    fn part_2(input: &str) -> u32 {
        super::part_2(input)
    }
}
