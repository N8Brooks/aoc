use std::str::FromStr;

use Instruction::*;
use Val::*;

pub fn part_1(input: &str) -> usize {
    evaluate_a(input, 0)
}

pub fn part_2(input: &str) -> usize {
    evaluate_a(input, 1)
}

fn evaluate_a(input: &str, c: usize) -> usize {
    let instructions: Vec<_> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut registers = [0; 4];
    registers[2] = c;
    let mut ip = 0;
    while let Some(instruction) = instructions.get(ip) {
        match *instruction {
            Cpy(Int(x), y) => registers[y] = x,
            Cpy(Reg(x), y) => registers[y] = registers[x],
            Inc(x) => registers[x] += 1,
            Dec(x) => registers[x] -= 1,
            Jnz(Int(x), y) if x != 0 => {
                ip = ip.strict_add_signed(y);
                continue;
            }
            Jnz(Reg(x), y) if registers[x] != 0 => {
                ip = ip.strict_add_signed(y);
                continue;
            }
            _ => {}
        }
        ip += 1;
    }
    registers[0]
}

#[derive(Copy, Clone)]
enum Instruction {
    Cpy(Val, usize),
    Inc(usize),
    Dec(usize),
    Jnz(Val, isize),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instruction, args) = s.split_once(' ').unwrap();
        match instruction {
            "cpy" => {
                let (arg1, arg2) = args.split_once(' ').unwrap();
                Ok(Cpy(arg1.parse().unwrap(), Val::reg(arg2)))
            }
            "inc" => Ok(Inc(Val::reg(args))),
            "dec" => Ok(Dec(Val::reg(args))),
            "jnz" => {
                let (arg1, arg2) = args.split_once(' ').unwrap();
                Ok(Jnz(arg1.parse().unwrap(), arg2.parse().unwrap()))
            }
            _ => Err(()),
        }
    }
}

/// Either an integer or the value of a register
#[derive(Copy, Clone)]
enum Val {
    Int(usize),
    Reg(usize),
}

impl FromStr for Val {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(i) = s.parse() {
            Ok(Int(i))
        } else {
            Ok(Reg(Val::reg(s)))
        }
    }
}

impl Val {
    fn reg(s: &str) -> usize {
        match s {
            "a" => 0,
            "b" => 1,
            "c" => 2,
            "d" => 3,
            _ => panic!("invalid register {s}"),
        }
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_12.txt");

    const EXAMPLE: &str = "cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a";

    #[test_case(EXAMPLE => 42)]
    #[test_case(INPUT => 318009)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(INPUT => 9227663)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
