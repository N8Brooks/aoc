use std::iter::{self, empty, once};

use itertools::{Itertools as _, multizip};
use num::Complex;

use crate::intcode::{IntcodeExt as _, parse_program};

pub fn part_1(input: &str) -> usize {
    let program = parse_program(input);
    let view = parse_view(program);
    alignment_sum(&view)
}

fn alignment_sum(map: &[Vec<u8>]) -> usize {
    map.array_windows()
        .enumerate()
        .flat_map(|(i, [a, b, c])| {
            multizip((a.array_windows(), b.array_windows(), c.array_windows()))
                .enumerate()
                .filter(|(_, ([_, n, _], [w, c, e], [_, s, _]))| {
                    *n == b'#' && *w == b'#' && *c == b'#' && *e == b'#' && *s == b'#'
                })
                .map(move |(j, _)| (i + 1) * (j + 1))
        })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    let mut program = parse_program(input);

    let view: Vec<_> = parse_view(program.clone());
    let at = |p: Complex<isize>| -> Option<u8> {
        let r: usize = p.re.try_into().ok()?;
        let c: usize = p.im.try_into().ok()?;
        view.get(r)?.get(c).copied()
    };

    let mut pos = view
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find_map(|(j, &b)| (b == b'^').then(|| Complex::new(i as isize, j as isize)))
        })
        .unwrap();
    let mut dir = Complex::new(-1, 0);
    let path: Vec<_> = iter::from_fn(|| {
        const R: Complex<isize> = Complex::new(0, -1);
        const L: Complex<isize> = Complex::new(0, 1);

        [(b'R', R), (b'L', L)]
            .into_iter()
            .find_map(|(turn, delta)| {
                let dir2 = dir * delta;
                let mut steps: u8 = 0;
                while at(pos + dir2) == Some(b'#') {
                    dir = dir2;
                    pos += dir;
                    steps += 1;
                }
                (steps > 0).then_some((turn, steps))
            })
    })
    .collect();

    program[0] = 2; // wake up robot

    let (main, functions) = compress(&path).unwrap();
    let main = main.into_iter().intersperse(b',').chain(once(b'\n'));
    let functions = functions.into_iter().flat_map(|f| {
        f.iter()
            .enumerate()
            .flat_map(|(i, &(turn, steps))| {
                let steps = [steps / 10 + b'0', steps % 10 + b'0']
                    .into_iter()
                    .skip_while(|&b| b == b'0');
                (i > 0)
                    .then_some(b',')
                    .into_iter()
                    .chain(once(turn))
                    .chain(once(b','))
                    .chain(steps)
            })
            .chain(once(b'\n'))
    });

    main.chain(functions)
        .chain(*b"n\n")
        .map(isize::from)
        .intcode(program)
        .last()
        .unwrap()
        .try_into()
        .unwrap()
}

fn parse_view(program: Vec<isize>) -> Vec<Vec<u8>> {
    let view: Vec<_> = empty()
        .intcode(program)
        .map(|x| x.try_into().unwrap())
        .collect();
    view.split(|&b| b == b'\n')
        .map(|line| line.to_vec())
        .collect()
}

fn compress<T: Copy + Eq + PartialEq>(tokens: &[T]) -> Option<(Vec<u8>, [&[T]; 3])> {
    let mut main = Vec::new();
    for a_len in 1..=tokens.len().min(5) {
        main.clear();
        let mut tokens = tokens;
        let a = &tokens[..a_len];
        while let Some(t) = tokens.strip_prefix(a) {
            main.push(b'A');
            tokens = t;
        }
        let i = main.len();
        for b_len in 1..=tokens.len().min(5) {
            main.truncate(i);
            let mut tokens = tokens;
            let b = &tokens[..b_len];
            while let Some((f, t)) = tokens
                .strip_prefix(b)
                .map(|t| (b'B', t))
                .or_else(|| tokens.strip_prefix(a).map(|t| (b'A', t)))
            {
                main.push(f);
                tokens = t;
            }
            let j = main.len();
            for c_len in 1..=tokens.len().min(5) {
                main.truncate(j);
                let mut tokens = tokens;
                let c = &tokens[..c_len];
                while let Some((f, t)) = tokens
                    .strip_prefix(c)
                    .map(|t| (b'C', t))
                    .or_else(|| tokens.strip_prefix(b).map(|t| (b'B', t)))
                    .or_else(|| tokens.strip_prefix(a).map(|t| (b'A', t)))
                {
                    main.push(f);
                    tokens = t;
                }
                if tokens.is_empty() {
                    return Some((main, [a, b, c]));
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "..#..........
..#..........
#######...###
#.#...#...#.#
#############
..#...#...#..
..#####...^..";

    #[test_case(EXAMPLE => 76)]
    fn alignment_sum(input: &str) -> usize {
        let view: Vec<_> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
        super::alignment_sum(&view)
    }

    const INPUT: &str = include_str!("../test_data/day_17.txt");

    #[test_case(INPUT => 11372)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(INPUT => 1155497)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
