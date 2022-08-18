use num::Complex;

pub fn part_1(input: &str) -> i32 {
    let Complex { re: pos, im: depth } = input
        .lines()
        .map(parse_command)
        .map(|(magnitude, direction)| -> Complex<i32> {
            match direction {
                "forward" => magnitude * Complex::new(1, 0),
                "up" => magnitude * Complex::new(0, -1),
                "down" => magnitude * Complex::new(0, 1),
                _ => panic!("Unknown direction encountered"),
            }
        })
        .sum();
    pos * depth
}

pub fn part_2(input: &str) -> i32 {
    let (mut aim, mut pos, mut depth) = (0, 0, 0);
    for line in input.lines() {
        let (magnitude, direction) = parse_command(line);
        match direction {
            "forward" => {
                pos += magnitude;
                depth += aim * magnitude;
            }
            "up" => aim -= magnitude,
            "down" => aim += magnitude,
            _ => panic!("Unknown direction encountered"),
        }
    }
    pos * depth
}

fn parse_command(line: &str) -> (i32, &str) {
    let (direction, magnitude) = line.split_once(' ').unwrap();
    let magnitude: i32 = magnitude.parse().unwrap();
    (magnitude, direction)
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};
    use lazy_static::lazy_static;
    use std::fs::read_to_string;
    use test_case::test_case;

    static EXAMPLE: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    lazy_static! {
        static ref INPUT: String = read_to_string("src/year_2021/testdata/day_02.txt").unwrap();
    }

    #[test_case(EXAMPLE, 150)]
    #[test_case(&INPUT, 1855814)]
    fn part_1_tests(input: &str, expected: i32) {
        assert_eq!(part_1(input), expected);
    }

    #[test_case(EXAMPLE, 900)]
    #[test_case(&INPUT, 1845455714)]
    fn part_2_tests(input: &str, expected: i32) {
        assert_eq!(part_2(input), expected);
    }
}
