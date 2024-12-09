use num::complex::Complex;

pub fn part_1(input: &str) -> isize {
    let Complex { re: hpos, im: vpos } = input
        .lines()
        .map(|line| {
            let (dir, mag) = line.split_once(' ').unwrap();
            let mag = mag.parse().unwrap();
            match dir {
                "forward" => Complex::new(mag, 0),
                "down" => Complex::new(0, mag),
                "up" => Complex::new(0, -mag),
                _ => panic!("invalid direction"),
            }
        })
        .fold(Complex::<isize>::new(0, 0), |a, b| a + b);
    hpos * vpos
}

pub fn part_2(input: &str) -> isize {
    let mut aim = 0;
    let mut pos = Complex::<isize>::new(0, 0);
    for line in input.lines() {
        let (dir, mag) = line.split_once(' ').unwrap();
        let mag = mag.parse().unwrap();
        match dir {
            "forward" => pos += Complex::new(mag, aim * mag),
            "down" => aim += mag,
            "up" => aim -= mag,
            _ => panic!("invalid direction"),
        }
    }
    let Complex { re: hpos, im: vpos } = pos;
    hpos * vpos
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    const EXAMPLE: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    const INPUT: &str = include_str!("../../../test_data/year_2021/day_02.txt");

    #[test_case(EXAMPLE, 150)]
    #[test_case(INPUT, 1855814)]
    fn part_1(input: &str, expected: isize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 900)]
    #[test_case(INPUT, 1845455714)]
    fn part_2(input: &str, expected: isize) {
        assert_eq!(super::part_2(input), expected);
    }
}
