use itertools::Itertools;

pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}

pub fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    static EXAMPLE: &str = "199
200
208
210
200
207
240
269
260
263";

    static INPUT: &str = include_str!("testdata/day_01.txt");

    #[test_case(EXAMPLE, 7)]
    #[test_case(INPUT, 1557)]
    fn part_2(input: &str, actual: usize) {
        assert_eq!(super::part_1(input), actual);
    }

    #[test_case(EXAMPLE, 5)]
    #[test_case(INPUT, 1608)]
    fn part_1(input: &str, actual: usize) {
        assert_eq!(super::part_2(input), actual);
    }
}
