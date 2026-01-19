use num::Complex;
use std::collections::HashSet;

pub fn part_1(input: &str) -> usize {
    simulate_rope(input, &mut [Complex::new(0, 0)])
}

pub fn part_2(input: &str) -> usize {
    simulate_rope(input, &mut [Complex::new(0, 0); 9])
}

fn simulate_rope(input: &str, knots: &mut [Complex<isize>]) -> usize {
    let mut head = Complex::new(0, 0);
    let mut seen: HashSet<Complex<isize>> =
        HashSet::from_iter(knots.last().map(|knot| knot.to_owned()));
    for (dir, count) in input.lines().map(parse_line) {
        for _ in 0..count {
            head += dir;
            let mut pre_knot = head;
            for cur_knot in knots.iter_mut() {
                let diff = pre_knot - *cur_knot;
                if diff.re.abs() > 1 || diff.im.abs() > 1 {
                    *cur_knot += Complex::new(diff.re.clamp(-1, 1), diff.im.clamp(-1, 1));
                }
                pre_knot = *cur_knot;
            }
            seen.insert(pre_knot);
        }
    }
    seen.len()
}

fn parse_line(line: &str) -> (Complex<isize>, usize) {
    let (dir, count) = line.split_once(' ').unwrap();
    let dir = get_dir(dir);
    let count = count.parse().unwrap();
    (dir, count)
}

fn get_dir(dir: &str) -> Complex<isize> {
    match dir.bytes().next().unwrap() {
        b'R' => Complex::new(1, 0),
        b'L' => Complex::new(-1, 0),
        b'U' => Complex::new(0, 1),
        b'D' => Complex::new(0, -1),
        dir => panic!("unknown dir {dir}"),
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE_1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const INPUT: &str = include_str!("../test_data/day_09.txt");

    #[test_case(EXAMPLE_1 => 13)]
    #[test_case(INPUT => 6190)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    const EXAMPLE_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test_case(EXAMPLE_1 => 1)]
    #[test_case(EXAMPLE_2 => 36)]
    #[test_case(INPUT => 2516)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
