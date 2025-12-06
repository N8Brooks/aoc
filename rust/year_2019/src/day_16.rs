use std::iter::{repeat_n, successors};

pub fn part_1(input: &str) -> String {
    "".to_string()
}

pub fn part_2(input: &str) -> usize {
    0
}

fn fft_phases(input: &str, n: usize) -> String {
    const BASE_PATTERN: [i8; 4] = [0, 1, 0, -1];
    let signal: Vec<_> = input.bytes().map(|b| b - b'0').collect();

    let signal = repeat_n((), n).fold(signal, |signal, _| {
        (1..=signal.len())
            .map(|count| {
                let pattern = BASE_PATTERN
                    .into_iter()
                    .cycle()
                    .flat_map(|p| repeat_n(p, count))
                    .skip(1);
                pattern
                    .zip(&signal)
                    .map(|(p, &s)| p * (s as i8))
                    .sum::<i8>()
                    .unsigned_abs()
                    % 10
            })
            .collect()
    });

    signal.into_iter().map(|d| (d + b'0') as char).collect()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    #[test_case("12345678", 4, "01029498")]
    fn fft_phases(input: &str, n: usize, expected: &str) {
        assert_eq!(super::fft_phases(input, n), expected);
    }

    const INPUT: &str = include_str!("../test_data/day_16.txt");

    const EXAMPLE_1: &str = "80871224585914546619083218645595";

    const EXAMPLE_2: &str = "19617804207202209144916044189917";

    const EXAMPLE_3: &str = "69317163492948606335995924319873";

    #[test_case(EXAMPLE_1, "24176176")]
    #[test_case(EXAMPLE_2, "73745418")]
    #[test_case(EXAMPLE_3, "52432133")]
    #[test_case(INPUT, "")]
    fn part_1(input: &str, expected: &str) {
        assert_eq!(super::part_1(input), expected);
    }

    // #[test_case(EXAMPLE_3, 82892753)]
    // #[test_case(EXAMPLE_4, 5586022)]
    // #[test_case(EXAMPLE_5, 460664)]
    #[test_case(INPUT, 0)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
