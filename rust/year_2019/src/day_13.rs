use std::{
    cell::Cell,
    iter::{empty, repeat_with},
    rc::Rc,
};

use hashbrown::HashSet;
use itertools::Itertools as _;
use num::Integer as _;

pub fn part_1(input: &str) -> usize {
    let program = parse_program(input);
    Intcode::new(program, empty())
        .tuples()
        .fold(HashSet::new(), |mut screen, (x, y, tile_id)| {
            if tile_id == 2 {
                screen.insert((x, y));
            }
            screen
        })
        .len()
}

pub fn part_2(input: &str) -> isize {
    let mut program = parse_program(input);
    program[0] = 2; // play for free

    let ball_x = Rc::new(Cell::new(0));
    let paddle_x = Rc::new(Cell::new(0));

    let inputs = {
        let bx = Rc::clone(&ball_x);
        let px = Rc::clone(&paddle_x);
        repeat_with(move || bx.cmp(&px) as isize)
    };

    Intcode::new(program, inputs)
        .tuples()
        .filter_map(|(x, y, tile_id)| {
            if x == -1 && y == 0 {
                return Some(tile_id);
            }
            match tile_id {
                3 => paddle_x.set(x),
                4 => ball_x.set(x),
                _ => {}
            }
            None
        })
        .last()
        .unwrap()
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

impl<I: Iterator<Item = isize>> Iterator for Intcode<I> {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
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
}

fn parse_program(input: &str) -> Vec<isize> {
    input.split(',').map(|num| num.parse().unwrap()).collect()
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

    const INPUT: &str = include_str!("../test_data/day_13.txt");

    #[test_case(INPUT => 326)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(INPUT => 15988)]
    fn part_2(input: &str) -> isize {
        super::part_2(input)
    }
}
