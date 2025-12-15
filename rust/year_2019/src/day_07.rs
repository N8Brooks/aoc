use std::iter::successors;

use itertools::Itertools as _;
use num::Integer as _;

pub fn part_1(input: &str) -> isize {
    let program = parse_program(input);
    (0..=4)
        .permutations(5)
        .map(|phases| {
            phases.into_iter().fold(0, |input, phase| {
                Intcode::new(&program, phase).run(input).unwrap()
            })
        })
        .max()
        .unwrap()
}

pub fn part_2(input: &str) -> isize {
    let program = parse_program(input);
    (5..=9)
        .permutations(5)
        .map(|phases| {
            let mut amps = phases
                .into_iter()
                .map(|phase| Intcode::new(&program, phase))
                .collect_array::<5>()
                .unwrap();
            successors(Some(0), |&input| {
                amps.iter_mut().try_fold(input, |input, amp| amp.run(input))
            })
            .last()
            .unwrap()
        })
        .max()
        .unwrap()
}

fn parse_program(input: &str) -> Vec<isize> {
    input.split(',').map(|num| num.parse().unwrap()).collect()
}

struct Intcode {
    /// Computer's memory
    memory: Vec<isize>,
    /// Instruction pointer
    ip: usize,
    /// Initial phase setting (consumed on first input)
    input_0: Option<isize>,
}

impl Intcode {
    fn new(program: &[isize], phase: isize) -> Self {
        Self {
            memory: program.to_vec(),
            ip: 0,
            input_0: Some(phase),
        }
    }

    /// Continues runing the program until it produces an output or halts.
    fn run(&mut self, input: isize) -> Option<isize> {
        loop {
            let instruction = self.next();
            let (modes, opcode) = instruction.div_rem(&100);
            let (mode_2, mode_1) = modes.div_rem(&10);
            match opcode {
                1 => {
                    let param_1 = self.read(mode_1);
                    let param_2 = self.read(mode_2);
                    self.write(param_1 + param_2);
                }
                2 => {
                    let param_1 = self.read(mode_1);
                    let param_2 = self.read(mode_2);
                    self.write(param_1 * param_2);
                }
                3 => {
                    let input = self.input_0.take().unwrap_or(input);
                    self.write(input);
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
                    self.write((param_1 < param_2).into());
                }
                8 => {
                    let param_1 = self.read(mode_1);
                    let param_2 = self.read(mode_2);
                    self.write((param_1 == param_2).into());
                }
                99 => return None,
                _ => panic!("unexpected opcode"),
            }
        }
    }

    fn read(&mut self, mode: isize) -> isize {
        let param = self.next();
        match mode {
            0 => {
                let i: usize = param.try_into().unwrap();
                self.memory[i]
            }
            1 => param,
            _ => panic!("unexpected mode {mode}"),
        }
    }

    fn write(&mut self, value: isize) {
        let i: usize = self.next().try_into().unwrap();
        self.memory[i] = value;
    }

    fn next(&mut self) -> isize {
        let word = self.memory[self.ip];
        self.ip += 1;
        word
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE_1_1: &str = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";

    const EXAMPLE_1_2: &str = "3,23,3,24,1002,24,10,24,1002,23,-1,23,\
101,5,23,23,1,24,23,23,4,23,99,0,0";

    const EXAMPLE_1_3: &str = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,\
1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";

    const INPUT: &str = include_str!("../test_data/day_07.txt");

    #[test_case(EXAMPLE_1_1 => 43210)]
    #[test_case(EXAMPLE_1_2 => 54321)]
    #[test_case(EXAMPLE_1_3 => 65210)]
    #[test_case(INPUT => 92663)]
    fn part_1(input: &str) -> isize {
        super::part_1(input)
    }

    const EXAMPLE_2_1: &str = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,\
27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";

    const EXAMPLE_2_2: &str = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,\
-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,\
53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";

    #[test_case(EXAMPLE_2_1 => 139629729)]
    #[test_case(EXAMPLE_2_2 => 18216)]
    #[test_case(INPUT => 14365052)]
    fn part_2(input: &str) -> isize {
        super::part_2(input)
    }
}
