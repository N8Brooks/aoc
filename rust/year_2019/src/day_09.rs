use num::Integer as _;

pub fn part_1(input: &str) -> isize {
    Intcode::new(input, 1).next().unwrap()
}

pub fn part_2(input: &str) -> isize {
    Intcode::new(input, 2).next().unwrap()
}

struct Intcode {
    /// Computer's memory
    memory: Vec<isize>,
    /// Instruction pointer
    ip: usize,
    /// Relative base
    rb: isize,
    /// Single input value
    input: Option<isize>,
}

impl Iterator for Intcode {
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
                    let input = self.input.take().unwrap();
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

impl Intcode {
    fn new(input: &str, input0: isize) -> Self {
        let program = input.split(',').map(|num| num.parse().unwrap()).collect();
        Self {
            memory: program,
            ip: 0,
            rb: 0,
            input: Some(input0),
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

    const EXAMPLE_1: &str = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";

    #[test]
    fn example_1() {
        let intcode = super::Intcode::new(EXAMPLE_1, 0);
        let expected = intcode.memory.clone();
        let actual: Vec<_> = intcode.collect();
        assert_eq!(actual, expected);
    }

    const EXAMPLE_2: &str = "1102,34915192,34915192,7,4,7,99,0";

    const EXAMPLE_3: &str = "104,1125899906842624,99";

    const INPUT: &str = include_str!("../test_data/day_09.txt");

    #[test_case(EXAMPLE_2 => 1219070632396864)]
    #[test_case(EXAMPLE_3 => 1125899906842624)]
    #[test_case(INPUT => 2171728567)]
    fn part_1(input: &str) -> isize {
        super::part_1(input)
    }

    #[test_case(INPUT => 49815)]
    fn part_2(input: &str) -> isize {
        super::part_2(input)
    }
}
