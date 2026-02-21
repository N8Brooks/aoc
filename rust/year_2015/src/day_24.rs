use itertools::Itertools;

pub fn part_1(input: &str) -> u64 {
    partition(input, 3)
}

pub fn part_2(input: &str) -> u64 {
    partition(input, 4)
}

fn partition(input: &str, k: u64) -> u64 {
    let weights = parse_weights(input);
    let target = weights.iter().sum::<u64>() / k;
    weights
        .into_iter()
        .powerset()
        .find(|combination| combination.iter().sum::<u64>() == target)
        .unwrap()
        .into_iter()
        .product()
}

fn parse_weights(input: &str) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_24.txt");

    const EXAMPLE: &str = "1
2
3
4
5
7
8
9
10
11";

    #[test_case(EXAMPLE => 99)]
    #[test_case(INPUT => 10439961859)]
    fn part_1(input: &str) -> u64 {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 44)]
    #[test_case(INPUT => 72050269)]
    fn part_2(input: &str) -> u64 {
        super::part_2(input)
    }
}
