use std::collections::VecDeque;

use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    let (steps, states) = parse_states(input);
    let mut tape = VecDeque::from([false]);
    let mut cursor = 0;
    let mut name = 0;

    for _ in 0..steps {
        let value = tape[cursor] as usize;
        let transition = states[name][value];

        tape[cursor] = transition.write;
        cursor = if let Some(cursor) = cursor.checked_add_signed(transition.move_) {
            if cursor == tape.len() {
                tape.push_back(false);
            }
            cursor
        } else {
            tape.push_front(false);
            0
        };
        name = transition.next;
    }

    tape.into_iter().filter(|&b| b).count()
}

fn parse_states(input: &str) -> (usize, Vec<[Transition; 2]>) {
    let (prefix, states) = input.split_once("\n\n").unwrap();
    let steps = prefix
        .lines()
        .nth(1)
        .unwrap()
        .strip_circumfix("Perform a diagnostic checksum after ", " steps.")
        .unwrap()
        .parse()
        .unwrap();
    let states = states.split("\n\n").map(parse_state).try_collect().unwrap();
    (steps, states)
}

fn parse_state(s: &str) -> Result<[Transition; 2], &'static str> {
    let mut lines = s.lines().skip(1);
    let [_, w0, m0, n0] = lines.next_array().ok_or("value 0")?;
    let [_, w1, m1, n1] = lines.next_array().ok_or("value 1")?;
    Ok([
        Transition::parse([w0, m0, n0])?,
        Transition::parse([w1, m1, n1])?,
    ])
}

#[derive(Debug, Clone, Copy)]
struct Transition {
    write: bool,
    move_: isize,
    next: usize,
}

impl Transition {
    fn parse([write, move_, next]: [&str; 3]) -> Result<Self, &'static str> {
        let write = match write.strip_circumfix("    - Write the value ", ".") {
            Some("0") => false,
            Some("1") => true,
            _ => return Err("invalid write"),
        };

        let move_ = match move_.strip_circumfix("    - Move one slot to the ", ".") {
            Some("left") => -1,
            Some("right") => 1,
            _ => return Err("invalid move"),
        };

        let [b @ b'A'..=b'Z'] = next
            .strip_circumfix("    - Continue with state ", ".")
            .ok_or("invalid next")?
            .as_bytes()
        else {
            return Err("invalid next");
        };

        Ok(Self {
            write,
            move_,
            next: (b - b'A') as usize,
        })
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_25.txt");

    const EXAMPLE: &str = "Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.";

    #[test_case(EXAMPLE => 3)]
    #[test_case(INPUT => 4225)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }
}
