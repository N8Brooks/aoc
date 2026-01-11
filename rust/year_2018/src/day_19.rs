use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    let (idx, instructions) = parse_input(input);
    let mut regs = [0; 6];
    while let Some(instruction) = instructions.get(regs[idx]) {
        regs = instruction.execute(regs);
        regs[idx] += 1;
    }
    regs[0]
}

pub fn part_2(input: &str) -> usize {
    // Find target number, add divisors
    let (idx, instructions) = parse_input(input);
    let mut regs = [0; 6];
    regs[0] = 1;
    for _ in 0..17 {
        let ip = regs[idx];
        regs = instructions[ip].execute(regs);
        regs[idx] += 1;
    }
    let target = regs.iter().max().unwrap();
    let divisor = (2..).find(|n| target % n == 0).unwrap();
    target + 1 + divisor + target / divisor
}

type Registers = [usize; 6];

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

use Op::*;

impl Instruction {
    fn execute(&self, mut regs: Registers) -> Registers {
        let [a, b, c] = self.vars;
        regs[c] = match self.op {
            Addr => regs[a] + regs[b],
            Addi => regs[a] + b,
            Mulr => regs[a] * regs[b],
            Muli => regs[a] * b,
            Banr => regs[a] & regs[b],
            Bani => regs[a] & b,
            Borr => regs[a] | regs[b],
            Bori => regs[a] | b,
            Setr => regs[a],
            Seti => a,
            Gtir => (a > regs[b]).into(),
            Gtri => (regs[a] > b).into(),
            Gtrr => (regs[a] > regs[b]).into(),
            Eqir => (a == regs[b]).into(),
            Eqri => (regs[a] == b).into(),
            Eqrr => (regs[a] == regs[b]).into(),
        };
        regs
    }
}

fn parse_input(input: &str) -> (usize, Vec<Instruction>) {
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
