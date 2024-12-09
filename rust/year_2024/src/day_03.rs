use std::sync::LazyLock;

use regex::Regex;

pub fn part_1(input: &str) -> usize {
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap());
    RE.captures_iter(input)
        .map(|cap| {
            let num_1 = cap[1].parse::<usize>().unwrap();
            let num_2 = cap[2].parse::<usize>().unwrap();
            num_1 * num_2
        })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(?:(do|don't)\(\))|mul\((\d{1,3}),(\d{1,3})\)").unwrap());
    let mut enabled = true;
    RE.captures_iter(input)
        .map(|cap| {
            if let Some(enable) = cap.get(1) {
                enabled = enable.as_str() == "do";
            } else if enabled {
                let num_1 = cap[2].parse::<usize>().unwrap();
                let num_2 = cap[3].parse::<usize>().unwrap();
                return num_1 * num_2;
            }
            0
        })
        .sum()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_03.txt");

    const EXAMPLE_1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test_case(EXAMPLE_1, 161)]
    #[test_case(INPUT, 175700056)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    const EXAMPLE_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test_case(EXAMPLE_2, 48)]
    #[test_case(INPUT, 71668682)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
