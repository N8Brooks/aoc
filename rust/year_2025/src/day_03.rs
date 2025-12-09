pub fn part_1(input: &str) -> usize {
    sum_max_joltage::<2>(input)
}

pub fn part_2(input: &str) -> usize {
    sum_max_joltage::<12>(input)
}

fn sum_max_joltage<const N: usize>(input: &str) -> usize
where
    [(); N + 1]:,
{
    input
        .lines()
        .map(|line| {
            let (init, rest) = line.as_bytes().split_at(N);
            let mut window = [b'0'; N + 1];
            window[..N].copy_from_slice(init);
            for &b in rest {
                window[N] = b;
                if let Some(i) = window.array_windows().position(|[a, b]| a < b) {
                    window.copy_within(i + 1.., i);
                }
            }
            window[..N]
                .iter()
                .fold(0, |acc, b| 10 * acc + usize::from(b - b'0'))
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
