use std::{mem, str::FromStr};

use crate::day_08::Instruction::*;

pub fn part_1(input: &str) -> isize {
    let code = parse_code(input);
    boot(&code).1
}

pub fn part_2(input: &str) -> isize {
    let mut code = parse_code(input);
    let n = code.len();
    (0..n)
        .find_map(|i| {
            let old = code[i];
            code[i] = match old {
                Nop(n) => Jmp(n),
                Jmp(n) => Nop(n),
                Acc(_) => return None,
            };
            let (ip, acc) = boot(&code);
            code[i] = old;
            (ip == n).then_some(acc)
        })
        .unwrap()
}

fn parse_code(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (op, arg) = s.split_once(' ').ok_or("no space")?;
        let arg = arg.parse().map_err(|_| "invalid arg")?;
        match op {
            "nop" => Ok(Nop(arg)),
            "acc" => Ok(Acc(arg)),
            "jmp" => Ok(Jmp(arg)),
            _ => Err("invalid op"),
        }
    }
}

fn boot(code: &[Instruction]) -> (usize, isize) {
    let mut unseen = vec![true; code.len()];
    let (mut ip, mut acc) = (0, 0);
    while unseen.get_mut(ip).is_some_and(|e| mem::replace(e, false)) {
        match code[ip] {
            Nop(_) => ip += 1,
            Acc(n) => {
                acc += n;
                ip += 1;
            }
            Jmp(n) => ip = ip.strict_add_signed(n),
        }
    }
    (ip, acc)
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_08.txt");

    const EXAMPLE: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test_case(EXAMPLE => 5)]
    #[test_case(INPUT => 1766)]
    fn part_1(input: &str) -> isize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 8)]
    #[test_case(INPUT => 1639)]
    fn part_2(input: &str) -> isize {
        super::part_2(input)
    }
}
