use itertools::Itertools;

pub fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count() as u32
}

pub fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    static EXAMPLE: &str = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";

    static INPUT: &str = include_str!("testdata/day_01.txt");

    #[test_case(EXAMPLE, 7)]
    #[test_case(INPUT, 1557)]
    fn part_1_examples(input: &str, expected: u32) {
        assert_eq!(part_1(input), expected);
    }

    #[test_case(EXAMPLE, 5)]
    #[test_case(INPUT, 1608)]
    fn part_2_examples(input: &str, expected: u32) {
        assert_eq!(part_2(input), expected);
    }
}
