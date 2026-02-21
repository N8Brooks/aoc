use Val::*;
use serde::Deserialize;
use std::cmp::{Ord, Ordering};

#[derive(Clone, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum Val {
    Arr(Vec<Val>),
    Num(u32),
}

impl PartialOrd for Val {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Val {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Num(a), Num(b)) => a.cmp(b),
            (&Num(a), b @ Arr(_)) => Arr(vec![Num(a)]).cmp(b),
            (a @ Arr(_), &Num(b)) => a.cmp(&Arr(vec![Num(b)])),
            (Arr(a), Arr(b)) => a
                .iter()
                .zip(b)
                .map(|(a, b)| a.cmp(b))
                .find(|&ord| ord != Ordering::Equal)
                .unwrap_or_else(|| a.len().cmp(&b.len())),
        }
    }
}

pub fn part_1(input: &str) -> usize {
    input
        .split("\n\n")
        .enumerate()
        .filter(|(_, pair)| {
            let (a, b) = pair.split_once('\n').unwrap();
            let a: Val = serde_json::from_str(a).unwrap();
            let b: Val = serde_json::from_str(b).unwrap();
            a < b
        })
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn part_2(input: &str) -> usize {
    let mut packets: Vec<_> = input
        .lines()
        .filter(|&line| !line.is_empty())
        .map(|line| serde_json::from_str::<Val>(line).unwrap())
        .collect();
    let divider_packets = [Arr(vec![Arr(vec![Num(2)])]), Arr(vec![Arr(vec![Num(6)])])];
    packets.extend_from_slice(&divider_packets);
    packets.sort_unstable();
    divider_packets
        .iter()
        .map(|divider_packet| packets.binary_search(divider_packet).unwrap() + 1)
        .product()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    const INPUT: &str = include_str!("../test_data/day_13.txt");

    #[test_case(EXAMPLE => 13)]
    #[test_case(INPUT => 5506)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 140)]
    #[test_case(INPUT => 21756)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
