use itertools::Itertools;

pub fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| -> i32 { line.parse().unwrap() })
        .tuple_windows()
        .filter(|&(a, b)| a < b)
        .count() as u32
}

pub fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| -> i32 { line.parse().unwrap() })
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|&(a, b)| a < b)
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};
    use lazy_static::lazy_static;
    use std::fs::read_to_string;
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

    lazy_static! {
        static ref INPUT: String = read_to_string("src/year_2021/testdata/day_01.txt").unwrap();
    }

    #[test_case(EXAMPLE, 7)]
    #[test_case(&INPUT, 1557)]
    fn part_1_tests(input: &str, expected: u32) {
        assert_eq!(part_1(input), expected);
    }

    #[test_case(EXAMPLE, 5)]
    #[test_case(&INPUT, 1608)]
    fn part_2_tests(input: &str, expected: u32) {
        assert_eq!(part_2(input), expected);
    }
}
