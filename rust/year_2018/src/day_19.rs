use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    let (idx, instructions) = parse_input(input);
    let mut device = Device::new(idx);
    while let Some(instruction) = instructions.get(device.regs[idx]) {
        device.execute(instruction);
    }
    device.regs[0]
}

pub fn part_2(input: &str) -> usize {
    // Find target number, add divisors
    let (idx, instructions) = parse_input(input);
    let mut device = Device::new(idx);
    device.regs[0] = 1;
    for _ in 0..17 {
        let ip = device.regs[idx];
        let instruction = &instructions[ip];
        device.execute(instruction);
    }
    let target = device.regs.iter().max().unwrap();
    let divisor = (2..).find(|&n| target.is_multiple_of(n)).unwrap();
    target + 1 + divisor + target / divisor
}

struct Device {
    idx: usize,
    regs: [usize; 6],
}

impl Device {
    fn new(idx: usize) -> Self {
        Self { idx, regs: [0; 6] }
    }

    fn execute(&mut self, Instruction { op, vars }: &Instruction) {
        use Op::*;
        let [a, b, c] = *vars;
        self.regs[c] = match op {
            Addr => self.regs[a] + self.regs[b],
            Addi => self.regs[a] + b,
            Mulr => self.regs[a] * self.regs[b],
            Muli => self.regs[a] * b,
            Banr => self.regs[a] & self.regs[b],
            Bani => self.regs[a] & b,
            Borr => self.regs[a] | self.regs[b],
            Bori => self.regs[a] | b,
            Setr => self.regs[a],
            Seti => a,
            Gtir => (a > self.regs[b]).into(),
            Gtri => (self.regs[a] > b).into(),
            Gtrr => (self.regs[a] > self.regs[b]).into(),
            Eqir => (a == self.regs[b]).into(),
            Eqri => (self.regs[a] == b).into(),
            Eqrr => (self.regs[a] == self.regs[b]).into(),
        };
        self.regs[self.idx] += 1;
    }
}

struct Instruction {
    op: Op,
    vars: [usize; 3],
}

enum Op {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

fn parse_input(input: &str) -> (usize, Vec<Instruction>) {
    use Op::*;
    let (idx, instructions) = input.split_once("\n").unwrap();
    let idx = idx.strip_prefix("#ip ").unwrap().parse().unwrap();
    let instructions = instructions
        .lines()
        .map(|line| {
            let (op, vars) = line.split_once(' ').unwrap();
            let op = match op {
                "addr" => Addr,
                "addi" => Addi,
                "mulr" => Mulr,
                "muli" => Muli,
                "banr" => Banr,
                "bani" => Bani,
                "borr" => Borr,
                "bori" => Bori,
                "setr" => Setr,
                "seti" => Seti,
                "gtir" => Gtir,
                "gtri" => Gtri,
                "gtrr" => Gtrr,
                "eqir" => Eqir,
                "eqri" => Eqri,
                "eqrr" => Eqrr,
                _ => panic!("unknown operation: {op}"),
            };
            let vars = vars
                .split(' ')
                .map(|s| s.parse().unwrap())
                .collect_array()
                .unwrap();
            Instruction { op, vars }
        })
        .collect();
    (idx, instructions)
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_19.txt");

    const EXAMPLE: &str = "#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5";

    #[test_case(EXAMPLE => 7)]
    #[test_case(INPUT => 1056)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(INPUT => 10915260)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
