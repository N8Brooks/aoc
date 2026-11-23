use std::str::{FromStr, Lines};

use arrayvec::ArrayVec;

type Sequence = ArrayVec<usize, 3>;

enum Rule {
    Byte(u8),
    Alternatives(ArrayVec<Sequence, 2>),
}

pub fn part_1(input: &str) -> usize {
    solve(input, false)
}

pub fn part_2(input: &str) -> usize {
    solve(input, true)
}

fn solve(input: &str, recursive: bool) -> usize {
    let (mut rules, messages) = parse_input(input);
    if recursive {
        rules.resize_with(rules.len().max(12), || None);
        rules[8] = Some(Rule::either(&[42], &[42, 8]));
        rules[11] = Some(Rule::either(&[42, 31], &[42, 11, 31]));
    }
    messages.filter(|message| is_valid(&rules, message)).count()
}

fn parse_input(input: &str) -> (Vec<Option<Rule>>, Lines<'_>) {
    let (definitions, messages) = input.split_once("\n\n").unwrap();
    let mut rules = Vec::new();
    for line in definitions.lines() {
        let (id, definition) = line.split_once(": ").unwrap();
        let id: usize = id.parse().unwrap();
        rules.resize_with(rules.len().max(id + 1), || None);
        rules[id] = Some(definition.parse().unwrap());
    }
    (rules, messages.lines())
}

fn is_valid(rules: &[Option<Rule>], message: &str) -> bool {
    // Each entry pairs a shared continuation with an optional alternative to retry.
    let mut stack = Vec::with_capacity(message.len().min(64) + 1);
    let root = ([0].iter(), message.as_bytes(), message.len());
    stack.push((([].iter(), 0), Some(root)));

    'search: while let Some((_, retry)) = stack.last_mut() {
        let Some((mut sequence, mut remaining, mut budget)) = retry.take() else {
            stack.pop();
            continue;
        };
        let mut parent = stack.len() - 1;
        while let Some(&id) = sequence.next() {
            if sequence.as_slice().is_empty() {
                let (resume, _) = &stack[parent];
                (sequence, parent) = resume.clone();
            }
            match rules[id].as_ref().unwrap() {
                Rule::Byte(byte) => {
                    let Some(rest) = remaining.strip_prefix(&[*byte]) else {
                        continue 'search;
                    };
                    remaining = rest;
                }
                Rule::Alternatives(alternatives) => {
                    // Each pending rule needs a byte. Replacing one rule with N costs N - 1 extra bytes.
                    let mut choices = alternatives.iter().filter_map(|ids| {
                        Some((ids.iter(), remaining, budget.checked_sub(ids.len())? + 1))
                    });
                    let Some(first) = choices.next() else {
                        continue 'search;
                    };
                    let retry = choices.next();
                    if retry.is_some() || !sequence.as_slice().is_empty() {
                        stack.push(((sequence, parent), retry));
                        parent = stack.len() - 1;
                    }
                    (sequence, remaining, budget) = first;
                }
            }
        }
        if remaining.is_empty() {
            return true;
        }
    }
    false
}

impl Rule {
    fn either(left: &[usize], right: &[usize]) -> Self {
        let choices = [left, right].map(|ids| ids.iter().copied().collect());
        Self::Alternatives(choices.into())
    }
}

impl FromStr for Rule {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let [b'"', byte, b'"'] = s.as_bytes() {
            return Ok(Self::Byte(*byte));
        }

        let mut alternatives = ArrayVec::new();
        for alternative in s.split(" | ") {
            let mut sequence = Sequence::new();
            for token in alternative.split(' ') {
                let id = token.parse().map_err(|_| "invalid rule reference")?;
                sequence
                    .try_push(id)
                    .map_err(|_| "too many rule references")?;
            }
            alternatives
                .try_push(sequence)
                .map_err(|_| "too many alternatives")?;
        }
        Ok(Self::Alternatives(alternatives))
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_19.txt");

    const EXAMPLE_1: &str = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;

    #[test_case(EXAMPLE_1 => 2)]
    #[test_case(INPUT => 162)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    const EXAMPLE_2: &str = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;

    #[test_case(EXAMPLE_2 => 12)]
    #[test_case(INPUT => 267)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
