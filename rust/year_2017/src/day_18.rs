use std::{
    iter::{self, empty},
    mem,
    num::NonZeroUsize,
    str::FromStr,
};

pub fn part_1(input: &str) -> isize {
    let instructions: Vec<Instruction> = input.lines().map(|line| line.parse().unwrap()).collect();
    Program::new(&instructions).run(empty()).last().unwrap()
}

pub fn part_2(input: &str) -> usize {
    let instructions: Vec<Instruction> = input.lines().map(|line| line.parse().unwrap()).collect();

    let mut a = Program::new(&instructions);
    let mut b = Program::new(&instructions);
    b.registers[(b'p' - b'a') as usize] = 1;

    let mut incoming = Vec::new();
    let mut outgoing = Vec::new();

    iter::from_fn(|| {
        let temp = a.run(incoming.drain(..));
        b.run(temp).collect_into(&mut outgoing);
        mem::swap(&mut incoming, &mut outgoing);
        NonZeroUsize::new(incoming.len())
    })
    .map(|len| len.get())
    .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Snd(Value), // should be isize - always int?
    Set(usize, Value),
    Add(usize, Value),
    Mul(usize, Value),
    Mod(usize, Value),
    Rcv(usize),
    Jgz(Value, Value),
}

use Instruction::*;

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (kind, args) = s.split_once(' ').ok_or(())?;

        match kind {
            "snd" => Ok(Snd(args.parse().map_err(|_| ())?)),
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
            "add" => {
                let (x, y) = args.split_once(' ').ok_or(())?;

                let x = if let [x] = x.as_bytes() {
                    (x - b'a') as usize
                } else {
                    return Err(());
                };
                let y = y.parse().map_err(|_| ())?;
                Ok(Add(x, y))
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
            "mod" => {
                let (x, y) = args.split_once(' ').ok_or(())?;
                let x = if let [x] = x.as_bytes() {
                    (x - b'a') as usize
                } else {
                    return Err(());
                };
                let y = y.parse().map_err(|_| ())?;
                Ok(Mod(x, y))
            }
            "rcv" => Ok(Rcv(if let [x] = args.as_bytes() {
                (x - b'a') as usize
            } else {
                return Err(());
            })),
            "jgz" => {
                let (x, y) = args.split_once(' ').ok_or(())?;
                let x = x.parse().map_err(|_| ())?;
                let y = y.parse().map_err(|_| ())?;
                Ok(Jgz(x, y))
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

use Value::*;

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

struct Program<'a> {
    instructions: &'a [Instruction],
    registers: [isize; 26],
    ip: usize,
}

impl Program<'_> {
    fn new<'a>(instructions: &'a [Instruction]) -> Program<'a> {
        Program {
            instructions,
            registers: [0; 26],
            ip: 0,
        }
    }

    fn run(&mut self, mut queue: impl Iterator<Item = isize>) -> impl Iterator<Item = isize> {
        iter::from_fn(move || {
            loop {
                match *self.instructions.get(self.ip)? {
                    Snd(x) => {
                        self.ip += 1;
                        return Some(self.registers.resolve(x));
                    }
                    Rcv(r) => self.registers[r] = queue.next()?,
                    Set(r, y) => self.registers[r] = self.registers.resolve(y),
                    Add(r, y) => self.registers[r] += self.registers.resolve(y),
                    Mul(r, y) => self.registers[r] *= self.registers.resolve(y),
                    Mod(r, y) => self.registers[r] %= self.registers.resolve(y),
                    Jgz(x, y) => {
                        if self.registers.resolve(x) > 0 {
                            self.ip = self.ip.strict_add_signed(self.registers.resolve(y));
                            continue;
                        }
                    }
                }
                self.ip += 1;
            }
        })
    }
}

trait Registers {
    fn resolve(&self, index: Value) -> isize;
}

impl Registers for [isize; 26] {
    fn resolve(&self, index: Value) -> isize {
        match index {
            Register(r) => self[r],
            Literal(x) => x,
        }
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_18.txt");

    const EXAMPLE: &str = "set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";

    #[test_case(EXAMPLE => 4)]
    #[test_case(INPUT => 7071)]
    fn part_1(input: &str) -> isize {
        super::part_1(input)
    }

    #[test_case(INPUT => 8001)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
