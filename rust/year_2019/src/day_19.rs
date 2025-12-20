use num::{Integer as _, One as _, Zero as _};

pub fn part_1(input: &str) -> usize {
    let program = parse_program(input);
    (0..50)
        .flat_map(|y| (0..50).map(move |x| [x, y]))
        .map(|inputs| {
            Intcode::new(program.clone(), inputs.into_iter())
                .next()
                .unwrap()
        })
        .filter(|&output| output == 1)
        .count()
}

pub fn part_2(input: &str) -> usize {
    let program = parse_program(input);
    let mut y = 0;
    (99..)
        .map(|x| {
            while Intcode::new(program.clone(), [x, y].into_iter())
                .next()
                .unwrap()
                .is_zero()
            {
                y += 1;
            }
            [x, y]
        })
        .find_map(|[x, y]| {
            Intcode::new(program.clone(), [x - 99, y + 99].into_iter())
                .next()
                .unwrap()
                .is_one()
                .then(|| (x - 99) * 10_000 + y)
        })
        .unwrap()
        .try_into()
        .unwrap()
}

fn parse_program(input: &str) -> Vec<isize> {
    input.split(',').map(|num| num.parse().unwrap()).collect()
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

    const INPUT: &str = include_str!("../test_data/day_19.txt");

    #[test_case(INPUT => 226)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(INPUT => 7900946)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
