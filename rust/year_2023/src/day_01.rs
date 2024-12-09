pub fn part_1(input: &str) -> usize {
    input.trim_end().lines().map(calibration_value_1).sum()
}

fn calibration_value_1(line: &str) -> usize {
    let first = line.bytes().find(|byte| byte.is_ascii_digit()).unwrap() - b'0';
    let last = line.bytes().rfind(|byte| byte.is_ascii_digit()).unwrap() - b'0';
    let value = first * 10 + last;
    value.into()
}

pub fn part_2(input: &str) -> usize {
    input.trim_end().lines().map(calibration_value_2).sum()
}

fn calibration_value_2(line: &str) -> usize {
    let func = |(i, byte): (usize, u8)| {
        byte.is_ascii_digit().then_some(byte - b'0').or_else(|| {
            (1..)
                .zip(NUMS)
                .find(|(_, num)| line[i..].starts_with(num))
                .map(|(j, _)| j)
        })
    };
    let first = line.bytes().enumerate().find_map(func).unwrap();
    let last = line.bytes().enumerate().rev().find_map(func).unwrap();
    (first * 10 + last) as usize
}

const NUMS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../../../test_data/year_2023/day_01.txt");

    const EXAMPLE_1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test_case(EXAMPLE_1, 142)]
    #[test_case(INPUT, 52974)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    const EXAMPLE_2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test_case(EXAMPLE_2, 281)]
    #[test_case(INPUT, 53340)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
