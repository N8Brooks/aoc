use std::iter;

use hashbrown::HashSet;
use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    iter_r0(input).next().unwrap()
}

pub fn part_2(input: &str) -> usize {
    let mut seen = HashSet::new();
    iter_r0(input)
        .take_while(|&r_cmp| seen.insert(r_cmp))
        .last()
        .unwrap()
}

fn iter_r0(input: &str) -> impl Iterator<Item = usize> {
    let (init, mask, mul) = parse_params(input);
    let mut r0 = 0;
    iter::from_fn(move || {
        let mut r2 = r0 | 65_536;
        r0 = init;
        loop {
            r0 += r2 & 255;
            r0 &= mask;
            r0 *= mul;
            r0 &= mask;
            if r2 < 256 {
                return Some(r0);
            }
            r2 /= 256;
        }
    })
}

fn parse_params(input: &str) -> (usize, usize, usize) {
    let (_, instructions) = input.split_once("\n").unwrap();
    instructions
        .lines()
        .map(|line| {
            let (op, vars) = line.split_once(' ').unwrap();
            let vars = vars
                .split(' ')
                .map(|s| s.parse().unwrap())
                .collect_array()
                .unwrap();
            (op, vars)
        })
        .tuple_windows()
        .find_map(|window| match window {
            (
                ("seti", [init, _, _]),
                ("bani", [_, 255, _]),
                ("addr", _),
                ("bani", [_, mask, _]),
                ("muli", [_, mul, _]),
            ) => Some((init, mask, mul)),
            _ => None,
        })
        .unwrap()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_21.txt");

    #[test_case(INPUT => 10846352)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(INPUT => 5244670)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
