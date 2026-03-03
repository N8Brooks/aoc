use std::{
    ops::{Index, IndexMut},
    str::FromStr,
};

use Instruction::*;

pub fn part_1(input: &str) -> u32 {
    evaluate_a(input, 0)
}

pub fn part_2(input: &str) -> u32 {
    evaluate_a(input, 1)
}

fn evaluate_a(input: &str, c: u32) -> u32 {
    let instructions: Vec<_> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut registers = Registers::new(0, 0, c, 0);
    let mut ip = 0;
    while let Some(&instruction) = instructions.get(ip) {
        match instruction {
            Cpy(x, y) => registers[y] = registers.get(x),
            Inc(x) => registers[x] += 1,
            Dec(x) => registers[x] -= 1,
            Jnz(x, y) => {
                if registers.get(x) != 0 {
                    ip = ip.strict_add_signed(y);
                    continue;
                }
            }
        }
        ip += 1;
    }
    registers[Ident::A]
}

#[derive(Copy, Clone)]
struct Registers([u32; 4]);

impl Registers {
    #[inline]
    fn new(a: u32, b: u32, c: u32, d: u32) -> Self {
        Self([a, b, c, d])
    }

    #[inline]
    fn get(&self, val: Value) -> u32 {
        match val {
            Value::Literal(x) => x,
            Value::Register(x) => self[x],
        }
    }
}

impl Index<Ident> for Registers {
    type Output = u32;

    #[inline]
    fn index(&self, index: Ident) -> &Self::Output {
        &self.0[usize::from(index)]
    }
}

impl IndexMut<Ident> for Registers {
    #[inline]
    fn index_mut(&mut self, index: Ident) -> &mut Self::Output {
        &mut self.0[usize::from(index)]
    }
}

#[derive(Copy, Clone)]
enum Instruction {
    Cpy(Value, Ident),
    Inc(Ident),
    Dec(Ident),
    Jnz(Value, isize),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instruction, args) = s.split_once(' ').unwrap();
        match instruction {
            "cpy" => {
                let (arg1, arg2) = args.split_once(' ').unwrap();
                Ok(Cpy(arg1.parse()?, arg2.parse()?))
            }
            "inc" => Ok(Inc(args.parse()?)),
            "dec" => Ok(Dec(args.parse()?)),
            "jnz" => {
                let (arg1, arg2) = args.split_once(' ').unwrap();
                Ok(Jnz(arg1.parse()?, arg2.parse().unwrap()))
            }
            _ => Err(()),
        }
    }
}

/// Either an integer or the value of a register
#[derive(Copy, Clone)]
enum Value {
    Literal(u32),
    Register(Ident),
}

impl FromStr for Value {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse()
            .map(Self::Literal)
            .or_else(|_| s.parse().map(Self::Register))
    }
}

#[repr(usize)]
#[derive(Copy, Clone)]
enum Ident {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}

impl FromStr for Ident {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Ident::*;
        match s {
            "a" => Ok(A),
            "b" => Ok(B),
            "c" => Ok(C),
            "d" => Ok(D),
            _ => Err(()),
        }
    }
}

impl From<Ident> for usize {
    #[inline]
    fn from(ident: Ident) -> Self {
        ident as usize
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
    fn part_1(input: &str) -> u32 {
        super::part_1(input)
    }

    #[test_case(INPUT => 9227663)]
    fn part_2(input: &str) -> u32 {
        super::part_2(input)
    }
}
