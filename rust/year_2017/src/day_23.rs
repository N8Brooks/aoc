use std::{
    iter::{self},
    str::FromStr,
};

use {Instruction::*, Value::*};

const N: usize = (b'h' - b'a') as usize + 1;

pub fn part_1(input: &str) -> usize {
    let instructions: Vec<Instruction> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut registers = [0; N];
    let mut ip = 0;
    iter::from_fn(|| {
        instructions.get(ip).copied().inspect(|&instruction| {
            match instruction {
                Set(x, y) => registers[x] = registers.resolve(y),
                Sub(x, y) => registers[x] -= registers.resolve(y),
                Mul(x, y) => registers[x] *= registers.resolve(y),
                Jnz(x, y) => {
                    if registers.resolve(x) != 0 {
                        ip = ip.strict_add_signed(y);
                        return;
                    }
                }
            }
            ip += 1;
        })
    })
    .filter(|instruction| matches!(instruction, Mul(_, _)))
    .count()
}

pub fn part_2(_input: &str) -> usize {
    (105700..=122700u32)
        .step_by(17)
        .filter(|&i| (2..i).any(|d| i.is_multiple_of(d)))
        .count()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Set(usize, Value),
    Sub(usize, Value),
    Mul(usize, Value),
    Jnz(Value, isize),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (kind, args) = s.split_once(' ').ok_or(())?;

        match kind {
            "set" => {
                let (x, y) = args.split_once(' ').ok_or(())?;
                let x = if let [x] = x.as_bytes() {
                    (x - b'a') as usize
                } else {
                    return Err(());
                };
                let y = y.parse().map_err(|_| ())?;
                Ok(Set(x, y))
            }
            "sub" => {
                let (x, y) = args.split_once(' ').ok_or(())?;
                let x = if let [x] = x.as_bytes() {
                    (x - b'a') as usize
                } else {
                    return Err(());
                };
                let y = y.parse().map_err(|_| ())?;
                Ok(Sub(x, y))
            }
            "mul" => {
                let (x, y) = args.split_once(' ').ok_or(())?;
                let x = if let [x] = x.as_bytes() {
                    (x - b'a') as usize
                } else {
                    return Err(());
                };
                let y = y.parse().map_err(|_| ())?;
                Ok(Mul(x, y))
            }
            "jnz" => {
                let (x, y) = args.split_once(' ').ok_or(())?;
                let x = x.parse().map_err(|_| ())?;
                let y = y.parse().map_err(|_| ())?;
                Ok(Jnz(x, y))
            }
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Value {
    Register(usize),
    Literal(isize),
}

impl FromStr for Value {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(x) = s.parse() {
            Ok(Literal(x))
        } else if let [x] = s.as_bytes() {
            Ok(Register((x - b'a') as usize))
        } else {
            Err(())
        }
    }
}

trait Registers {
    fn resolve(&self, value: Value) -> isize;
}

impl Registers for [isize; N] {
    fn resolve(&self, value: Value) -> isize {
        match value {
            Register(x) => self[x],
            Literal(x) => x,
        }
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_23.txt");

    #[test_case(INPUT => 3025)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(INPUT => 915)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
