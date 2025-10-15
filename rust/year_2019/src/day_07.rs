use itertools::Itertools as _;

pub fn part_1(input: &str) -> isize {
    let program = parse_program(input);
    (0..=4)
        .permutations(5)
        .map(|phases| {
            phases.into_iter().fold(0, |input, phase| {
                Intcode::new(&program, phase).run(input).unwrap()
                // intcode(&program, phase, input.into_iter())
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
                .map(|p| Intcode::new(&program, p))
                .collect_array::<5>()
                .unwrap();
            let mut input = 0;
            let mut i = 0;
            while let Some(output) = amps[i].run(input) {
                input = output;
                i = if i < 4 { i + 1 } else { 0 }
            }
            input
        })
        .max()
        .unwrap()
}

fn parse_program(input: &str) -> Vec<isize> {
    input.split(',').map(|num| num.parse().unwrap()).collect()
}

struct Intcode {
    memory: Vec<isize>,
    ip: usize,
    init: Option<isize>,
}

impl Intcode {
    fn new(program: &[isize], init: isize) -> Self {
        Self {
            memory: program.to_vec(),
            ip: 0,
            init: Some(init),
        }
    }

    fn run(&mut self, input: isize) -> Option<isize> {
        loop {
            let value = self.next();
            let op = value % 100;
            match op {
                1 | 2 | 7 | 8 => {
                    let (l, r) = self.read2(value);
                    let value = match op {
                        1 => l + r,
                        2 => l * r,
                        7 => (l < r) as isize,
                        8 => (l == r) as isize,
                        _ => unreachable!(),
                    };
                    self.write(value);
                }
                5 | 6 => {
                    let (cond, target) = self.read2(value);
                    if op == 5 && cond != 0 || op == 6 && cond == 0 {
                        self.ip = target.try_into().unwrap();
                    }
                }
                3 => {
                    let value = self.init.take().unwrap_or(input);
                    self.write(value);
                }
                4 => return Some(self.read1(value)),
                99 => return None,
                _ => panic!("unexpected opcode"),
            }
        }
    }

    fn read1(&mut self, value: isize) -> isize {
        self.read((value / 100) % 10)
    }

    fn read2(&mut self, value: isize) -> (isize, isize) {
        (self.read((value / 100) % 10), self.read(value / 1000))
    }

    fn read(&mut self, mode: isize) -> isize {
        let value = self.next();
        if mode == 0 {
            let i: usize = value.try_into().unwrap();
            self.memory[i]
        } else {
            value
        }
    }

    fn next(&mut self) -> isize {
        let value = self.memory[self.ip];
        self.ip += 1;
        value
    }

    fn write(&mut self, value: isize) {
        let ip: usize = self.next().try_into().unwrap();
        self.memory[ip] = value;
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

    #[test_case(EXAMPLE_1_1, 43210)]
    #[test_case(EXAMPLE_1_2, 54321)]
    #[test_case(EXAMPLE_1_3, 65210)]
    #[test_case(INPUT, 92663)]
    fn part_1(input: &str, expected: isize) {
        assert_eq!(super::part_1(input), expected);
    }

    const EXAMPLE_2_1: &str = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,\
27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";

    const EXAMPLE_2_2: &str = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,\
-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,\
53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";

    #[test_case(EXAMPLE_2_1, 139629729)]
    #[test_case(EXAMPLE_2_2, 18216)]
    #[test_case(INPUT, 14365052)]
    fn part_2(input: &str, expected: isize) {
        assert_eq!(super::part_2(input), expected);
    }
}
