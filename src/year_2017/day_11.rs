use num::complex::Complex;

fn direction(dir: &str) -> Complex<i32> {
    match dir {
        "nw" => Complex::new(1, 0),
        "n" => Complex::new(0, 1),
        "ne" => Complex::new(-1, 1),
        "se" => Complex::new(-1, 0),
        "s" => Complex::new(0, -1),
        "sw" => Complex::new(1, -1),
        _ => panic!("Invalid direction"),
    }
}

fn distance(c: Complex<i32>) -> i32 {
    (c.im.abs() + (c.im + c.re).abs() + c.re.abs()) / 2
}

pub fn part_1(input: &str) -> i32 {
    distance(input.split(',').map(direction).sum())
}

pub fn part_2(input: &str) -> i32 {
    input
        .split(',')
        .map(direction)
        .scan(Complex::new(0, 0), |sum, c| {
            *sum += c;
            Some(*sum)
        })
        .map(distance)
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};
    use std::fs::read_to_string;

    #[test]
    fn part_1_example_1() {
        assert_eq!(part_1("ne,ne,ne"), 3);
    }

    #[test]
    fn part_1_example_2() {
        assert_eq!(part_1("ne,ne,sw,sw"), 0);
    }

    #[test]
    fn part_1_example_3() {
        assert_eq!(part_1("ne,ne,s,s"), 2);
    }

    #[test]
    fn part_1_example_4() {
        assert_eq!(part_1("se,sw,se,sw,sw"), 3);
    }

    #[test]
    fn part_1_input() {
        let input = read_to_string("src/year_2017/testdata/day_11.txt").unwrap();
        assert_eq!(part_1(&input), 784);
    }

    #[test]
    fn part_2_example_1() {
        assert_eq!(part_2("ne,ne,ne"), 3);
    }

    #[test]
    fn part_2_example_2() {
        assert_eq!(part_2("ne,ne,sw,sw"), 2);
    }

    #[test]
    fn part_2_example_3() {
        assert_eq!(part_2("ne,ne,s,s"), 2);
    }

    #[test]
    fn part_2_example_4() {
        assert_eq!(part_2("se,sw,se,sw,sw"), 3);
    }

    #[test]
    fn part_2_input() {
        let input = read_to_string("src/year_2017/testdata/day_11.txt").unwrap();
        assert_eq!(part_2(&input), 1558);
    }
}
