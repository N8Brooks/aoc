use Instruction::*;
use num::{Integer as _, One as _};

pub fn part_1(input: &str) -> u32 {
    run(input, 0)
}

pub fn part_2(input: &str) -> u32 {
    run(input, 1)
}

fn run(input: &str, reg_a: u32) -> u32 {
    let instructions = parse_instructions(input);
    let mut registers = [reg_a, 0];
    let mut pc: usize = 0;
    while let Some(instruction) = instructions.get(pc) {
        let jump = match *instruction {
            Hlf(r) => {
                registers[r] /= 2;
                1
            }
            Tpl(r) => {
                registers[r] *= 3;
                1
            }
            Inc(r) => {
                registers[r] += 1;
                1
            }
            Jmp(offset) => offset,
            Jie(r, offset) => {
                if registers[r].is_even() {
                    offset
                } else {
                    1
                }
            }
            Jio(r, offset) => {
                if registers[r].is_one() {
                    offset
                } else {
                    1
                }
            }
        };
        pc = pc.strict_add_signed(jump)
    }
    registers[B]
}

type Register = usize;

const A: Register = 0;
const B: Register = 1;

#[derive(Copy, Clone)]
enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(isize),
    Jie(Register, isize),
    Jio(Register, isize),
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    fn reg(reg: &str) -> Register {
        match reg {
            "a" => A,
            "b" => B,
            _ => panic!("invalid register: {reg}"),
        }
    }
    input
        .lines()
        .map(|line| {
            let (op, rest) = line.split_once(' ').unwrap();
            match op {
                "hlf" => Hlf(reg(rest)),
                "tpl" => Tpl(reg(rest)),
                "inc" => Inc(reg(rest)),
                "jmp" => Jmp(rest.parse().unwrap()),
                "jie" => {
                    let (r, offset) = rest.split_once(", ").unwrap();
                    Jie(reg(r), offset.parse().unwrap())
                }
                "jio" => {
                    let (r, offset) = rest.split_once(", ").unwrap();
                    Jio(reg(r), offset.parse().unwrap())
                }
                _ => panic!("invalid instruction: {line}"),
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_23.txt");

    #[test_case(INPUT => 307)]
    fn part_1(input: &str) -> u32 {
        super::part_1(input)
    }

    #[test_case(INPUT => 160)]
    fn part_2(input: &str) -> u32 {
        super::part_2(input)
    }
}
