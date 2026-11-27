use std::{iter, num::NonZeroU64};

use self::{Bracket::*, Token::*};

pub fn part_1(input: &str) -> u64 {
    input
        .lines()
        .map(BracketStack::analyze)
        .filter_map(Result::err)
        .map(Bracket::error)
        .sum()
}

pub fn part_2(input: &str) -> u64 {
    let mut scores: Vec<_> = input
        .lines()
        .map(BracketStack::analyze)
        .filter_map(Result::ok)
        .filter_map(BracketStack::completion)
        .collect();

    let mid = scores.len() / 2;
    scores.select_nth_unstable(mid).1.get()
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
struct BracketStack(NonZeroU64);

impl BracketStack {
    fn analyze(s: &str) -> Result<Self, Bracket> {
        s.bytes().try_fold(Self::new(), |mut stack, b| {
            match Token::parse(b) {
                Open(bracket) => stack.push(bracket),
                Close(bracket) if stack.pop() != Some(bracket) => return Err(bracket),
                Close(_) => {}
            }

            Ok(stack)
        })
    }
}

impl BracketStack {
    const fn new() -> Self {
        Self(NonZeroU64::MIN)
    }

    fn push(&mut self, bracket: Bracket) {
        const FOUR: NonZeroU64 = NonZeroU64::new(4).unwrap();
        self.0 = self.0.checked_mul(FOUR).expect("stack overflow") | bracket as u64;
    }

    fn pop(&mut self) -> Option<Bracket> {
        let bits = self.0.get();
        self.0 = NonZeroU64::new(bits >> 2)?;
        Some(Bracket::from_low_bits(bits))
    }

    fn completion(mut self) -> Option<NonZeroU64> {
        const FIVE: NonZeroU64 = NonZeroU64::new(5).unwrap();
        iter::from_fn(|| self.pop())
            .map(Bracket::completion)
            .reduce(|score, value| score.strict_mul(FIVE).strict_add(value.get()))
    }
}

#[derive(Debug)]
enum Token {
    Open(Bracket),
    Close(Bracket),
}

impl Token {
    fn parse(b: u8) -> Self {
        match b {
            b'(' => Open(Paren),
            b')' => Close(Paren),
            b'[' => Open(Square),
            b']' => Close(Square),
            b'{' => Open(Curly),
            b'}' => Close(Curly),
            b'<' => Open(Angle),
            b'>' => Close(Angle),
            _ => panic!("invalid token: {}", b as char),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bracket {
    Paren = 0,
    Square = 1,
    Curly = 2,
    Angle = 3,
}

impl Bracket {
    const fn from_low_bits(bits: u64) -> Self {
        [Paren, Square, Curly, Angle][(bits & 3) as usize]
    }

    const fn error(self) -> u64 {
        [3, 57, 1197, 25137][self as usize]
    }

    const fn completion(self) -> NonZeroU64 {
        NonZeroU64::new(self as u64 + 1).unwrap()
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_10.txt");

    const EXAMPLE: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test_case(EXAMPLE => 26397)]
    #[test_case(INPUT => 370407)]
    fn part_1(input: &str) -> u64 {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 288957)]
    #[test_case(INPUT => 3249889609)]
    fn part_2(input: &str) -> u64 {
        super::part_2(input)
    }
}
