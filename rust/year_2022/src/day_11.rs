use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;

#[derive(Debug)]
struct Monkey {
    /// Activity level for a monkey
    activity: u64,
    /// Worry level for each item
    items: VecDeque<u64>,
    /// Variables for a quatratic equation
    operation: (u64, u64, u64),
    /// Divisibility test factor
    divisor: u64,
    /// Monkey when test is true
    true_monkey_index: usize,
    /// Monkey when test is false
    false_monkey_index: usize,
}

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"Monkey \d+:
  Starting items: (?P<items>[\d, ]+)
  Operation: new = (?P<left>old|\d+) (?P<op>\+|\*) (?P<right>old|\d+)
  Test: divisible by (?P<divisor>\d+)
    If true: throw to monkey (?P<is_true_monkey>\d+)
    If false: throw to monkey (?P<is_false_monkey>\d+)"
    )
    .unwrap();
}

impl Monkey {
    fn inspect(&mut self, item: u64) -> u64 {
        self.activity += 1;
        self.operation.0 * item * item + self.operation.1 * item + self.operation.2
    }

    fn get_test_index(&self, item: u64) -> usize {
        if item.is_multiple_of(self.divisor) {
            self.true_monkey_index
        } else {
            self.false_monkey_index
        }
    }
}

impl From<&str> for Monkey {
    fn from(input: &str) -> Self {
        let cap = RE.captures(input).unwrap();
        Monkey {
            activity: 0,
            items: cap["items"]
                .split(", ")
                .map(|item| item.parse().unwrap())
                .collect(),
            operation: match (&cap["left"], &cap["op"], &cap["right"]) {
                ("old", "*", "old") => (1, 0, 0),
                ("old", "*", b) => (0, b.parse().unwrap(), 0),
                ("old", "+", c) => (0, 1, c.parse().unwrap()),
                (left, op, right) => panic!("unrecognized pattern {left} {op} {right}"),
            },
            divisor: cap["divisor"].parse().unwrap(),
            true_monkey_index: cap["is_true_monkey"].parse().unwrap(),
            false_monkey_index: cap["is_false_monkey"].parse().unwrap(),
        }
    }
}

fn simulate_rounds<F: Fn(u64) -> u64>(
    mut monkeys: Vec<Monkey>,
    n_rounds: usize,
    worry_func: F,
) -> u64 {
    for _ in 0..n_rounds {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                let item = worry_func(monkeys[i].inspect(item));
                let j = monkeys[i].get_test_index(item);
                monkeys[j].items.push_back(item);
            }
        }
    }
    monkeys.select_nth_unstable_by(1, |a, b| b.activity.cmp(&a.activity));
    monkeys[0].activity * monkeys[1].activity
}

pub fn part_1(input: &str) -> u64 {
    let monkeys: Vec<_> = input.split("\n\n").map(Monkey::from).collect();
    simulate_rounds(monkeys, 20, |item| item / 3)
}

pub fn part_2(input: &str) -> u64 {
    let monkeys: Vec<_> = input.split("\n\n").map(Monkey::from).collect();
    let m: u64 = monkeys.iter().map(|monkey| monkey.divisor).product();
    simulate_rounds(monkeys, 10_000, |item| item % m)
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    const INPUT: &str = include_str!("../../../test_data/year_2022/day_11.txt");

    #[test_case(EXAMPLE, 10605)]
    #[test_case(INPUT, 56120)]
    fn part_1(input: &str, expected: u64) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 2713310158)]
    #[test_case(INPUT, 24389045529)]
    fn part_2(input: &str, expected: u64) {
        assert_eq!(super::part_2(input), expected);
    }
}
