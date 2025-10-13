pub fn part_1(input: &str) -> isize {
    const ID: isize = 1;
    Intcode::new(input).intcode_2(ID)
}

pub fn part_2(input: &str) -> isize {
    const ID: isize = 5;
    Intcode::new(input).intcode_2(ID)
}

fn parse_program(input: &str) -> Vec<isize> {
    input.split(',').map(|num| num.parse().unwrap()).collect()
}

struct Intcode {
    program: Vec<isize>,
    i: usize,
}

impl Intcode {
    fn new(input: &str) -> Self {
        Self {
            program: parse_program(input),
            i: 0,
        }
    }

    fn intcode_2(mut self, input: isize) -> isize {
        let mut output = -1;
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
                        self.i = target.try_into().unwrap();
                    }
                }
                3 => self.write(input),
                4 => output = self.read1(value),
                99 => return output,
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
            self.program[i]
        } else {
            value
        }
    }

    fn next(&mut self) -> isize {
        let value = self.program[self.i];
        self.i += 1;
        value
    }

    fn write(&mut self, value: isize) {
        let i: usize = self.next().try_into().unwrap();
        self.program[i] = value;
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
