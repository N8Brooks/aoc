use std::iter::once;

use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    sum_max_joltage::<2>(input)
}

pub fn part_2(input: &str) -> usize {
    sum_max_joltage::<12>(input)
}

fn sum_max_joltage<const N: usize>(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (window, rest) = line.as_bytes().split_at(N);
            let mut window: [_; N] = window.try_into().unwrap();
            for &b in rest {
                if let Some(i) = window
                    .into_iter()
                    .chain(once(b))
                    .tuple_windows()
                    .position(|(a, c)| c > a)
                {
                    window[i..].rotate_left(1);
                    window[N - 1] = b;
                }
            }
            window
                .into_iter()
                .fold(0, |acc, b| 10 * acc + (b - b'0') as usize)
        })
        .sum()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_03.txt");

    const EXAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test_case(EXAMPLE => 357)]
    #[test_case(INPUT => 16887)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 3121910778619)]
    #[test_case(INPUT => 167302518850275)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
