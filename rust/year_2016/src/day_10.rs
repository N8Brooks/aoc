use Place::*;
use std::{num::NonZeroU32, str::FromStr};

pub fn part_1(input: &str) -> usize {
    const TARGET: (NonZeroU32, NonZeroU32) =
        (NonZeroU32::new(17).unwrap(), NonZeroU32::new(61).unwrap());
    let (mut stack, rules) = parse_input(input);
    let mut bots = [None; 210];
    let mut bins = [None; 21];
    while let Some((i, a)) = stack.pop() {
        if let Some(b) = bots[i].take() {
            let (a, b) = if a < b { (a, b) } else { (b, a) };
            if (a, b) == TARGET {
                return i;
            }
            let (lo, hi) = rules[i].unwrap();
            match lo {
                Bot(i) => stack.push((i, a)),
                Bin(i) => bins[i] = Some(a),
            }
            match hi {
                Bot(i) => stack.push((i, b)),
                Bin(i) => bins[i] = Some(b),
            }
        } else {
            bots[i] = Some(a);
        }
    }
    panic!("no bot found that compares 17 and 61")
}

pub fn part_2(input: &str) -> NonZeroU32 {
    let (mut stack, rules) = parse_input(input);
    let mut bots = [None; 210];
    let mut bins = [None; 21];
    while let Some((i, a)) = stack.pop() {
        if let Some(b) = bots[i].take() {
            let (a, b) = if a < b { (a, b) } else { (b, a) };
            let (lo, hi) = rules[i].unwrap();
            match lo {
                Bot(i) => stack.push((i, a)),
                Bin(i) => bins[i] = Some(a),
            }
            match hi {
                Bot(i) => stack.push((i, b)),
                Bin(i) => bins[i] = Some(b),
            }
        } else {
            bots[i] = Some(a);
        }
    }
    let [Some(a), Some(b), Some(c), ..] = bins else {
        panic!("not all bins filled");
    };
    a.checked_mul(b).unwrap().checked_mul(c).unwrap()
}

type Places = (Place, Place);

fn parse_input(input: &str) -> (Vec<(usize, NonZeroU32)>, [Option<Places>; 210]) {
    let mut stack = Vec::new();
    let mut rules = [None; 210];
    for line in input.lines() {
        if let Some(line) = line.strip_prefix("value ") {
            let (value, bot) = line.split_once(" goes to bot ").unwrap();
            stack.push((bot.parse().unwrap(), value.parse().unwrap()));
        } else if let Some(line) = line.strip_prefix("bot ") {
            let (bot, rest) = line.split_once(" gives low to ").unwrap();
            let (low, high) = rest.split_once(" and high to ").unwrap();
            let bot: usize = bot.parse().unwrap();
            rules[bot] = Some((low.parse().unwrap(), high.parse().unwrap()));
        } else {
            panic!("invalid input line: {line}");
        }
    }
    (stack, rules)
}

#[derive(Debug, Clone, Copy)]
enum Place {
    Bot(usize),
    Bin(usize),
}

impl FromStr for Place {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(bot) = s.strip_prefix("bot ") {
            Ok(Bot(bot.parse().unwrap()))
        } else if let Some(bin) = s.strip_prefix("output ") {
            Ok(Bin(bin.parse().unwrap()))
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_10.txt");

    const EXAMPLE: &str = "value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2";

    #[test_case(INPUT => 98)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 30)]
    #[test_case(INPUT => 4042)]
    fn part_2(input: &str) -> u32 {
        super::part_2(input).get()
    }
}
