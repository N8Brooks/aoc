use num::Integer as _;

pub(crate) fn parse_program(input: &str) -> Vec<isize> {
    input.split(',').map(|num| num.parse().unwrap()).collect()
}

#[derive(Debug)]
pub(crate) struct Intcode<I: Iterator<Item = isize>> {
    /// Computer's memory
    pub(crate) memory: Vec<isize>,
    /// Instruction pointer
    pub(crate) ip: usize,
    /// Relative base
    pub(crate) rb: isize,
    /// Input iterator
    pub(crate) inputs: I,
}

pub(crate) trait IntcodeExt: Iterator<Item = isize> + Sized {
    fn intcode(self, program: Vec<isize>) -> Intcode<Self>;
}

impl<I: Iterator<Item = isize>> IntcodeExt for I {
    fn intcode(self, program: Vec<isize>) -> Intcode<I> {
        Intcode::new(program, self)
    }
}

impl<I: Iterator<Item = isize>> Iterator for Intcode<I> {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        self.step()
    }
}

impl<I: Iterator<Item = isize>> Intcode<I> {
    pub(crate) fn new(memory: Vec<isize>, inputs: I) -> Self {
        Self {
            memory,
            ip: 0,
            rb: 0,
            inputs,
        }
    }

    pub(crate) fn step(&mut self) -> Option<isize> {
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
                    if let Some(input) = self.inputs.next() {
                        self.write(input, mode_1);
                    } else {
                        // This is specifically for Day 23
                        self.write(-1, mode_1);
                        return None;
                    }
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

    pub(crate) fn read(&mut self, mode: isize) -> isize {
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

    pub(crate) fn write(&mut self, value: isize, mode: isize) {
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

    pub(crate) fn fetch(&mut self) -> isize {
        let word = self.memory[self.ip];
        self.ip += 1;
        word
    }
}
