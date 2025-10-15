use num::Integer as _;

pub fn part_1(input: &str) -> isize {
    const ID: isize = 1;
    Intcode::new(input).run(ID).unwrap()
}

pub fn part_2(input: &str) -> isize {
    const ID: isize = 5;
    Intcode::new(input).run(ID).unwrap()
}

struct Intcode {
    /// Computer's memory
    memory: Vec<isize>,
    /// Instruction pointer
    ip: usize,
}

impl Intcode {
    fn new(input: &str) -> Self {
        let memory = input.split(',').map(|num| num.parse().unwrap()).collect();
        Self { memory, ip: 0 }
    }

    /// Returns the last output produced before halting.
    fn run(&mut self, input: isize) -> Option<isize> {
        let mut output = None;
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
                    self.write(input);
                }
                4 => output = Some(self.read(mode_1)),
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
                99 => return output,
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
        let ip: usize = self.next().try_into().unwrap();
        self.memory[ip] = value;
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

    const INPUT: &str = include_str!("../test_data/day_05.txt");

    #[test_case(INPUT, 9006673)]
    fn part_1(input: &str, expected: isize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(INPUT, 3629692)]
    fn part_2(input: &str, expected: isize) {
        assert_eq!(super::part_2(input), expected);
    }
}
