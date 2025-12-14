use std::{iter::repeat_n, ops::Rem as _};

pub fn part_1(input: &str) -> usize {
    let signal: Vec<_> = input.bytes().map(|b| b - b'0').collect();
    (0..100)
        .fold(signal, |signal, _| {
            const BASE_PATTERN: [isize; 4] = [0, 1, 0, -1];
            (1..=signal.len())
                .map(|count| {
                    let pattern = BASE_PATTERN
                        .into_iter()
                        .cycle()
                        .flat_map(|p| repeat_n(p, count))
                        .skip(1);
                    signal
                        .iter()
                        .map(|&s| s as isize)
                        .zip(pattern)
                        .map(|(s, p)| s * p)
                        .sum::<isize>()
                        .abs()
                        .rem(10) as u8
                })
                .collect()
        })
        .into_iter()
        .take(8)
        .fold(0, |acc, d| acc * 10 + d as usize)
}

pub fn part_2(input: &str) -> usize {
    let signal: Vec<_> = input.bytes().map(|b| b - b'0').collect();
    let offset = input[..7].parse().unwrap();
    let total_len = signal.len() * 10_000;
    let mut signal: Vec<_> = signal
        .into_iter()
        .cycle()
        .take(total_len)
        .skip(offset)
        .collect();
    for _ in 0..100 {
        let mut suffix_sum = 0;
        for v in signal.iter_mut().rev() {
            suffix_sum = (suffix_sum + *v) % 10;
            *v = suffix_sum;
        }
    }
    signal
        .into_iter()
        .take(8)
        .fold(0, |acc, d| acc * 10 + d as usize)
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_16.txt");

    const EXAMPLE_1: &str = "80871224585914546619083218645595";
    const EXAMPLE_2: &str = "19617804207202209144916044189917";
    const EXAMPLE_3: &str = "69317163492948606335995924319873";

    #[test_case(EXAMPLE_1 => 24176176)]
    #[test_case(EXAMPLE_2 => 73745418)]
    #[test_case(EXAMPLE_3 => 52432133)]
    #[test_case(INPUT => 74369033)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    const EXAMPLE_4: &str = "03036732577212944063491565474664";
    const EXAMPLE_5: &str = "02935109699940807407585447034323";
    const EXAMPLE_6: &str = "03081770884921959731165446850517";

    #[test_case(EXAMPLE_4 => 84462026)]
    #[test_case(EXAMPLE_5 => 78725270)]
    #[test_case(EXAMPLE_6 => 53553731)]
    #[test_case(INPUT => 19903864)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
