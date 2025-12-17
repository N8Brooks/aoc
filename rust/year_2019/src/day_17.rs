use std::iter::{self, empty, once};

use itertools::{Itertools as _, intersperse, multizip};
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
        map.get(r).and_then(|row| row.get(c)).copied()
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
    let path = iter::from_fn(|| {
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
            (steps > 0).then(|| format!("{turn},{steps}"))
        })
    });
    let path: Vec<_> = path.collect();

    let (pattern, a, b, c) = compress(&path).unwrap();
    let pattern = pattern.into_iter().intersperse(',').chain(once('\n'));
    let a = a
        .into_iter()
        .intersperse(",".into())
        .chain(once("\n".into()));
    let b = b
        .into_iter()
        .intersperse(",".into())
        .chain(once("\n".into()));
    let c = c
        .into_iter()
        .intersperse(",".into())
        .chain(once("\n".into()));
    let inputs = once(pattern.collect::<String>())
        .chain(a)
        .chain(b)
        .chain(c)
        .chain(once("n\n".into()))
        .collect::<String>();
    let inputs = inputs.bytes().map(|b| b as isize);

    program[0] = 2; // wake up robot
    Intcode::new(program, inputs)
        .last()
        .unwrap()
        .try_into()
        .unwrap()
}

fn compress(tokens: &[String]) -> Option<(Vec<char>, Vec<String>, Vec<String>, Vec<String>)> {
    let mut pattern = Vec::new();
    for a_len in 1..=5 {
        let mut tokens = tokens;
        pattern.clear();
        let a = &tokens[..a_len];
        while tokens.starts_with(a) {
            pattern.push('A');
            tokens = &tokens[a_len..];
        }
        for b_len in 1..=5 {
            let mut tokens = tokens;
            let b = &tokens[..b_len];
            let mut pattern = pattern.clone();
            while tokens.starts_with(a) || tokens.starts_with(b) {
                if tokens.starts_with(a) {
                    pattern.push('A');
                    tokens = &tokens[a_len..];
                } else {
                    pattern.push('B');
                    tokens = &tokens[b_len..];
                }
            }
            for c_len in 1..=5 {
                let mut tokens = tokens;
                let c = &tokens[..c_len];
                let mut pattern = pattern.clone();
                while tokens.starts_with(a) || tokens.starts_with(b) || tokens.starts_with(c) {
                    if tokens.starts_with(a) {
                        pattern.push('A');
                        tokens = &tokens[a_len..];
                    } else if tokens.starts_with(b) {
                        pattern.push('B');
                        tokens = &tokens[b_len..];
                    } else {
                        pattern.push('C');
                        tokens = &tokens[c_len..];
                    }
                }
                if tokens.is_empty()
                    && pattern.len() <= 20
                    && a.len() <= 20
                    && b.len() <= 20
                    && c.len() <= 20
                {
                    return Some((pattern.to_vec(), a.to_vec(), b.to_vec(), c.to_vec()));
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
