pub fn part_1(input: &str) -> u32 {
    let bit_length = input.bytes().position(|char| char == b'\n').unwrap();
    let mut length = 0;
    let pop_counts = input
        .lines()
        .fold(vec![0; bit_length], |mut pop_counts, line| {
            line.bytes().enumerate().for_each(|(i, bit)| {
                pop_counts[i] += (bit - b'0') as u32;
            });
            length += 1;
            pop_counts
        });
    let (mut gamma, mut epsilon) = (0, 0);
    pop_counts.iter().for_each(|&pop_count| {
        let bit = (pop_count > length / 2) as u32;
        gamma = gamma << 1 | bit;
        epsilon = epsilon << 1 | 1 - bit;
    });
    gamma * epsilon
}

pub fn part_2(input: &str) -> u32 {
    let bit_length = input.bytes().position(|char| char == b'\n').unwrap() - 1;
    let mut zeros: Vec<u32> = input
        .lines()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect();
    let mut ones = zeros.clone();
    let mut i = 1 << bit_length;
    while zeros.len() > 1 {
        let pop_count = zeros.iter().filter(|num| *num & i > 0).count();
        let bit = 2 * pop_count < zeros.len();
        zeros = zeros
            .into_iter()
            .filter(|num| (num & i > 0) == bit)
            .collect();
        i >>= 1;
    }
    let mut i = 1 << bit_length;
    while ones.len() > 1 {
        let pop_count = ones.iter().filter(|num| *num & i > 0).count();
        let bit = 2 * pop_count >= ones.len();
        ones = ones
            .into_iter()
            .filter(|num| (num & i > 0) == bit)
            .collect();
        i >>= 1;
    }
    zeros[0] * ones[0]
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};
    use lazy_static::lazy_static;
    use std::fs::read_to_string;
    use test_case::test_case;

    static EXAMPLE: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    lazy_static! {
        static ref INPUT: String = read_to_string("src/year_2021/testdata/day_03.txt").unwrap();
    }

    #[test_case(EXAMPLE, 198)]
    #[test_case(&INPUT, 3242606)]
    fn part_1_tests(input: &str, expected: u32) {
        assert_eq!(part_1(input), expected);
    }

    #[test_case(EXAMPLE, 230)]
    #[test_case(&INPUT, 4856080)]
    fn part_2_tests(input: &str, expected: u32) {
        assert_eq!(part_2(input), expected);
    }
}
