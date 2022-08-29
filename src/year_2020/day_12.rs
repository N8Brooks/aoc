use num::{traits::Pow, Complex};

pub fn part_1(input: &str) -> i32 {
    let mut location = Complex::new(0, 0);
    let mut direction = Complex::new(1, 0);
    input
        .lines()
        .map(|line| line.split_at(1))
        .for_each(|(action, value)| {
            let value: i32 = value.parse().unwrap();
            match action {
                "N" => location.im += value,
                "S" => location.im -= value,
                "E" => location.re += value,
                "W" => location.re -= value,
                "L" => direction *= Complex::new(0, 1).pow(value / 90),
                "R" => direction *= Complex::new(0, -1).pow(value / 90),
                "F" => location += direction * value,
                _ => panic!("Unknown action encountered"),
            }
        });
    location.re.abs() + location.im.abs()
}

pub fn part_2(input: &str) -> i32 {
    let mut location = Complex::new(0, 0);
    let mut waypoint = Complex::new(10, 1);
    input
        .lines()
        .map(|line| line.split_at(1))
        .for_each(|(action, value)| {
            let value: i32 = value.parse().unwrap();
            match action {
                "N" => waypoint.im += value,
                "S" => waypoint.im -= value,
                "E" => waypoint.re += value,
                "W" => waypoint.re -= value,
                "L" => waypoint *= Complex::new(0, 1).pow(value / 90),
                "R" => waypoint *= Complex::new(0, -1).pow(value / 90),
                "F" => location += waypoint * value,
                _ => panic!("Unknown action encountered"),
            }
        });
    location.re.abs() + location.im.abs()
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};
    use lazy_static::lazy_static;
    use std::fs::read_to_string;
    use test_case::test_case;

    static EXAMPLE: &str = "F10
N3
F7
R90
F11";

    lazy_static! {
        static ref INPUT: String = read_to_string("src/year_2020/testdata/day_12.txt").unwrap();
    }

    #[test_case(EXAMPLE, 25)]
    #[test_case(&INPUT, 2847)]
    fn part_1_tests(input: &str, expected: i32) {
        assert_eq!(part_1(input), expected);
    }

    #[test_case(EXAMPLE, 286)]
    #[test_case(&INPUT, 29839)]
    fn part_2_tests(input: &str, expected: i32) {
        assert_eq!(part_2(input), expected);
    }
}
