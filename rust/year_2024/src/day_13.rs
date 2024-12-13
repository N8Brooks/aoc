use std::sync::LazyLock;

use num::Integer as _;
use regex::Regex;

pub fn part_1(input: &str) -> i64 {
    answer(input, 0)
}

pub fn part_2(input: &str) -> i64 {
    answer(input, 10000000000000)
}

fn answer(input: &str, conversion_error: i64) -> i64 {
    const PATTERN: &str = r"Button A: X\+([0-9]+), Y\+([0-9]+)
Button B: X\+([0-9]+), Y\+([0-9]+)
Prize: X=([0-9]+), Y=([0-9]+)";
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(PATTERN).unwrap());
    RE.captures_iter(input)
        .filter_map(|cap| {
            let [ax, ay, bx, by, px, py] = [&cap[1], &cap[2], &cap[3], &cap[4], &cap[5], &cap[6]]
                .map(|s| s.parse::<i64>().unwrap());
            let px = px + conversion_error;
            let py = py + conversion_error;
            let det = ax * by - ay * bx; // Could check for 0
            let x_det = px * by - py * bx;
            if !x_det.is_multiple_of(&det) {
                return None;
            };
            let y_det = ax * py - ay * px;
            x_det.is_multiple_of(&det).then(|| {
                let x = x_det / det;
                let y = y_det / det;
                3 * x + y
            })
        })
        .sum()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    const INPUT: &str = include_str!("../test_data/day_13.txt");

    #[test_case(EXAMPLE, 480)]
    #[test_case(INPUT, 29877)]
    fn part_1(input: &str, expected: i64) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 875318608908)]
    #[test_case(INPUT, 99423413811305)]
    fn part_2(input: &str, expected: i64) {
        assert_eq!(super::part_2(input), expected);
    }
}
