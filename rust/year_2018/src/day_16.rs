use std::array;

use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    let (samples, _program) = parse_input(input);

    samples
        .filter(|&(before, (_, a, b, c), after)| {
            OPERATIONS
                .into_iter()
                .filter(|op| op(before, a, b, c) == after)
                .next_chunk::<3>()
                .is_ok()
        })
        .count()
}

pub fn part_2(input: &str) -> usize {
    let (samples, program) = parse_input(input);

    let mut operations: [Vec<_>; OPERATIONS.len()] =
        array::from_fn(|_| (0..OPERATIONS.len()).collect());

    for (before, (opcode, a, b, c), after) in samples {
        operations[opcode].retain(|&op| OPERATIONS[op](before, a, b, c) == after);
    }

    let mut graph: [_; OPERATIONS.len()] = array::from_fn(|_| Vec::with_capacity(OPERATIONS.len()));
    for (opcode, ops) in operations.iter().enumerate() {
        for &op in ops {
            graph[op].push(opcode);
        }
    }

    let mut stack: Vec<_> = operations
        .iter()
        .filter(|ops| ops.len() == 1)
        .map(|ops| ops[0])
        .collect();

    while let Some(op) = stack.pop() {
        for &opcode in &graph[op] {
            let ops = &mut operations[opcode];
            if ops.len() == 1 {
                continue;
            }
            ops.swap_remove(ops.iter().position(|&x| x == op).unwrap());
            if ops.len() == 1 {
                stack.push(ops[0]);
            }
        }
    }

    let operations = operations.map(|ops| {
        let op = ops.into_iter().exactly_one().unwrap();
        OPERATIONS[op]
    });

    program.fold([0; 4], |regs, (opcode, a, b, c)| {
        operations[opcode](regs, a, b, c)
    })[0]
}

type Registers = [usize; 4];

type Instruction = (usize, usize, usize, usize);

fn parse_input(
    input: &str,
) -> (
    impl Iterator<Item = (Registers, Instruction, Registers)>,
    impl Iterator<Item = Instruction>,
) {
    let (samples, program) = input.split_once("\n\n\n\n").unwrap_or((input, ""));
    let samples = samples.split("\n\n").map(|sample| {
        let (before, instruction, after) = sample.lines().collect_tuple().unwrap();
        let before: [_; 4] = before
            .strip_circumfix("Before: [", "]")
            .unwrap()
            .split(", ")
            .map(|n| n.parse().unwrap())
            .collect_array()
            .unwrap();
        let (opcode, a, b, c) = instruction
            .split(" ")
            .map(|n| n.parse().unwrap())
            .collect_tuple()
            .unwrap();
        let after: [_; 4] = after
            .strip_circumfix("After:  [", "]")
            .unwrap()
            .split(", ")
            .map(|n| n.parse().unwrap())
            .collect_array()
            .unwrap();
        (before, (opcode, a, b, c), after)
    });
    let program = program.lines().map(|line| {
        line.split(" ")
            .map(|n| n.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap()
    });
    (samples, program)
}

const OPERATIONS: [fn([usize; 4], usize, usize, usize) -> [usize; 4]; 16] = [
    addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri, eqrr,
];

fn addr(mut regs: [usize; 4], a: usize, b: usize, c: usize) -> [usize; 4] {
    regs[c] = regs[a] + regs[b];
    regs
}

fn addi(mut regs: [usize; 4], a: usize, b: usize, c: usize) -> [usize; 4] {
    regs[c] = regs[a] + b;
    regs
}

fn mulr(mut regs: [usize; 4], a: usize, b: usize, c: usize) -> [usize; 4] {
    regs[c] = regs[a] * regs[b];
    regs
}

fn muli(mut regs: [usize; 4], a: usize, b: usize, c: usize) -> [usize; 4] {
    regs[c] = regs[a] * b;
    regs
}

fn banr(mut regs: [usize; 4], a: usize, b: usize, c: usize) -> [usize; 4] {
    regs[c] = regs[a] & regs[b];
    regs
}

fn bani(mut regs: [usize; 4], a: usize, b: usize, c: usize) -> [usize; 4] {
    regs[c] = regs[a] & b;
    regs
}

fn borr(mut regs: [usize; 4], a: usize, b: usize, c: usize) -> [usize; 4] {
    regs[c] = regs[a] | regs[b];
    regs
}

fn bori(mut regs: [usize; 4], a: usize, b: usize, c: usize) -> [usize; 4] {
    regs[c] = regs[a] | b;
    regs
}

fn setr(mut regs: [usize; 4], a: usize, _b: usize, c: usize) -> [usize; 4] {
    regs[c] = regs[a];
    regs
}

fn seti(mut regs: [usize; 4], a: usize, _b: usize, c: usize) -> [usize; 4] {
    regs[c] = a;
    regs
}

fn gtir(mut regs: [usize; 4], a: usize, b: usize, c: usize) -> [usize; 4] {
    regs[c] = if a > regs[b] { 1 } else { 0 };
    regs
}

fn gtri(mut regs: [usize; 4], a: usize, b: usize, c: usize) -> [usize; 4] {
    regs[c] = if regs[a] > b { 1 } else { 0 };
    regs
}

fn gtrr(mut regs: [usize; 4], a: usize, b: usize, c: usize) -> [usize; 4] {
    regs[c] = if regs[a] > regs[b] { 1 } else { 0 };
    regs
}

fn eqir(mut regs: [usize; 4], a: usize, b: usize, c: usize) -> [usize; 4] {
    regs[c] = if a == regs[b] { 1 } else { 0 };
    regs
}

fn eqri(mut regs: [usize; 4], a: usize, b: usize, c: usize) -> [usize; 4] {
    regs[c] = if regs[a] == b { 1 } else { 0 };
    regs
}

fn eqrr(mut regs: [usize; 4], a: usize, b: usize, c: usize) -> [usize; 4] {
    regs[c] = if regs[a] == regs[b] { 1 } else { 0 };
    regs
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_16.txt");

    const EXAMPLE_1: &str = "Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]



";

    #[test_case(EXAMPLE_1 => 1)]
    #[test_case(INPUT => 560)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(INPUT => 622)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
