use std::iter::{self, empty, once};

use itertools::{Itertools as _, multizip};
use num::{Complex, Integer as _};

pub fn part_1(input: &str) -> usize {
    let program = parse_program(input);
    let view: Vec<_> = Intcode::new(program, empty()).map(|x| x as u8).collect();
    parse_view(&String::from_utf8(view).unwrap())
}

fn parse_view(s: &str) -> usize {
    s.lines()
        .map(|line| line.as_bytes())
        .tuple_windows()
        .enumerate()
        .flat_map(|(i, (a, b, c))| {
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

    let view: Vec<_> = Intcode::new(program.clone(), empty())
        .map(|x| x as u8)
        .collect();
    let view = String::from_utf8(view).unwrap();
    let map = view.lines().map(|line| line.as_bytes()).collect::<Vec<_>>();
    let at = |p: Complex<isize>| -> Option<u8> {
        let r: usize = p.re.try_into().ok()?;
        let c: usize = p.im.try_into().ok()?;
        map.get(r)?.get(c).copied()
    };

    let mut pos = map
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

        [('R', R), ('L', L)].into_iter().find_map(|(turn, delta)| {
            let dir2 = dir * delta;
            let mut steps = 0;
            while at(pos + dir2) == Some(b'#') {
                dir = dir2;
                pos += dir;
                steps += 1;
            }
            (steps > 0).then_some((turn, steps))
        })
    })
    .collect();

    let (main, functions) = compress(&path).unwrap();
    let main = main.into_iter().intersperse(b',').chain(once(b'\n'));
    let functions = functions.into_iter().flat_map(|f| {
        f.iter()
            .map(|&(turn, steps)| format!("{turn},{steps}"))
            .intersperse(",".to_string())
            .flat_map(|s| s.into_bytes())
            .chain(once(b'\n'))
    });
    let inputs = main.chain(functions).chain(*b"n\n").map(|b| b as isize);

    program[0] = 2; // wake up robot
    Intcode::new(program, inputs)
        .last()
        .unwrap()
        .try_into()
        .unwrap()
}

fn compress<T: Copy + Eq + PartialEq>(tokens: &[T]) -> Option<(Vec<u8>, [&[T]; 3])> {
    let mut main = Vec::new();
    for a_len in 1..=5 {
        main.clear();
        let mut tokens = tokens;
        let a = &tokens[..a_len];
        while let Some(t) = tokens.strip_prefix(a) {
            main.push(b'A');
            tokens = t;
        }
        let i = main.len();
        for b_len in 1..=5 {
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
            for c_len in 1..=5 {
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

struct Intcode<I: Iterator<Item = isize>> {
    /// Computer's memory
    memory: Vec<isize>,
    /// Instruction pointer
    ip: usize,
    /// Relative base
    rb: isize,
    /// Input iterator
    inputs: I,
}

fn parse_program(input: &str) -> Vec<isize> {
    input.split(',').map(|num| num.parse().unwrap()).collect()
}

impl<I: Iterator<Item = isize>> Iterator for Intcode<I> {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

impl<I: Iterator<Item = isize>> Intcode<I> {
    fn new(memory: Vec<isize>, inputs: I) -> Self {
        Self {
            memory,
            ip: 0,
            rb: 0,
            inputs,
        }
    }

    fn next(&mut self) -> Option<isize> {
        loop {
            let instruction = self.fetch();
            let (modes, opcode) = instruction.div_rem(&100);
            let (modes, mode_1) = modes.div_rem(&10);
            let (mode_3, mode_2) = modes.div_rem(&10);
            match opcode {
                1 => {
                    let param_1 = self.read(mode_1);
                    let param_2 = self.read(mode_2);
                    self.write(param_1 + param_2, mode_3);
                }
                2 => {
                    let param_1 = self.read(mode_1);
                    let param_2 = self.read(mode_2);
                    self.write(param_1 * param_2, mode_3);
                }
                3 => {
                    let input = self.inputs.next().unwrap();
                    self.write(input, mode_1);
                }
                4 => return Some(self.read(mode_1)),
                5 => {
                    let param_1 = self.read(mode_1);
                    let param_2 = self.read(mode_2);
                    if param_1 != 0 {
                        self.ip = param_2.try_into().unwrap();
                    }
                }
                6 => {
                    let param_1 = self.read(mode_1);
                    let param_2 = self.read(mode_2);
                    if param_1 == 0 {
                        self.ip = param_2.try_into().unwrap();
                    }
                }
                7 => {
                    let param_1 = self.read(mode_1);
                    let param_2 = self.read(mode_2);
                    self.write((param_1 < param_2).into(), mode_3);
                }
                8 => {
                    let param_1 = self.read(mode_1);
                    let param_2 = self.read(mode_2);
                    self.write((param_1 == param_2).into(), mode_3);
                }
                9 => {
                    let param_1 = self.read(mode_1);
                    self.rb += param_1;
                }
                99 => return None,
                _ => panic!("unexpected opcode {opcode}"),
            }
        }
    }

    fn read(&mut self, mode: isize) -> isize {
        let word = self.fetch();
        let i = match mode {
            0 => word,
            1 => return word,
            2 => self.rb + word,
            _ => panic!("unexpected mode {mode}"),
        };
        let u: usize = i.try_into().unwrap();
        self.memory.get(u).copied().unwrap_or(0)
    }

    fn write(&mut self, value: isize, mode: isize) {
        let word = self.fetch();
        let i = match mode {
            0 => word,
            2 => self.rb + word,
            _ => panic!("unexpected mode {mode}"),
        };
        let u: usize = i.try_into().unwrap();
        if u >= self.memory.len() {
            self.memory.resize(u + 1, 0);
        }
        self.memory[u] = value;
    }

    fn fetch(&mut self) -> isize {
        let word = self.memory[self.ip];
        self.ip += 1;
        word
    }
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
    fn parse_view(input: &str) -> usize {
        super::parse_view(input)
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
