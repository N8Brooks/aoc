use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    const NOUN: usize = 12;
    const VERB: usize = 2;
    Intcode::new(input).compute(NOUN, VERB)
}

pub fn part_2(input: &str) -> usize {
    const TARGET: usize = 19690720;
    let mut program = Intcode::new(input);
    let (noun, verb) = (0..=99)
        .cartesian_product(0..=99)
        .find(|&(noun, verb)| {
            program.compute(noun, verb) == TARGET
        })
        .unwrap();
    100 * noun + verb
}

struct Intcode {
    /// Original program
    program: Vec<usize>,
    /// Computer's memory
    memory: Vec<usize>,
    /// Instruction pointer
    ip: usize,
}

impl Intcode {
    fn new(input: &str) -> Self {
        let program: Vec<_> = input.split(',').map(|num| num.parse().unwrap()).collect();
        let memory = vec![0; program.len()];
        Self { program, memory, ip: 0 }
    }

    fn compute(&mut self, noun: usize, verb: usize) -> usize {
        self.program[1] = noun;
        self.program[2] = verb;
        self.run();
        self.memory[0]
    }

    fn run(&mut self) {
        self.memory.copy_from_slice(&self.program);
        self.ip = 0;
        loop {
            let opcode = self.next();
            match opcode {
                1 | 2 => {
                    let param_1 = self.read();
                    let param_2 = self.read();
                    let i = self.next();
                    self.memory[i] = if opcode == 1 { param_1 + param_2 } else { param_1 * param_2 };
                }
                99 => return,
                _ => panic!("unexpected opcode"),
            }
        }
    }

    fn read(&mut self) -> usize {
        let param = self.next();
        self.memory[param]
    }
    
    fn next(&mut self) -> usize {
        let num = self.memory[self.ip];
        self.ip += 1;
        num
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    #[test_case("1,9,10,3,2,3,11,0,99,30,40,50" => vec![3500,9,10,70,2,3,11,0,99,30,40,50])]
    #[test_case("1,0,0,0,99" => vec![2,0,0,0,99])]
    #[test_case("2,3,0,3,99" => vec![2,3,0,6,99])]
    #[test_case("2,4,4,5,99,0" => vec![2,4,4,5,99,9801])]
    #[test_case("1,1,1,4,99,5,6,0,99" => vec![30,1,1,4,2,5,6,0,99])]
    fn intcode(input: &str) -> Vec<usize> {
        let mut intcode = super::Intcode::new(input);
        intcode.run();
        intcode.memory
    }

    const INPUT: &str = include_str!("../test_data/day_02.txt");

    #[test_case(INPUT => 3760627)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(INPUT => 7195)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
