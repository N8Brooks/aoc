use std::cmp::Ordering;

use serde_json::{
    json,
    Value::{self, Array, Number},
};

fn value_cmp(a: &Value, b: &Value) -> Ordering {
    match (a, b) {
        (Number(a), Number(b)) => a.as_u64().unwrap().cmp(&b.as_u64().unwrap()),
        (a @ Number(_), b @ Array(_)) => value_cmp(&Array(vec![a.to_owned()]), b),
        (a @ Array(_), b @ Number(_)) => value_cmp(a, &Array(vec![b.to_owned()])),
        (Array(a), Array(b)) => a
            .iter()
            .zip(b)
            .find_map(|(a, b)| match value_cmp(a, b) {
                Ordering::Equal => None,
                ordering => Some(ordering),
            })
            .unwrap_or_else(|| a.len().cmp(&b.len())),
        _ => panic!("Unexpected comparison"),
    }
}

pub fn part_1(input: &str) -> usize {
    input
        .split("\n\n")
        .enumerate()
        .filter(|(_, pair)| {
            let (a, b) = pair.split_once('\n').unwrap();
            let a = serde_json::from_str(a).unwrap();
            let b = serde_json::from_str(b).unwrap();
            value_cmp(&a, &b) != Ordering::Greater
        })
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn part_2(input: &str) -> usize {
    let mut packets: Vec<_> = input
        .lines()
        .filter(|&line| !line.is_empty())
        .map(|line| serde_json::from_str::<Value>(line).unwrap())
        .collect();
    let divider_packets = [json!([[2]]), json!([[6]])];
    packets.extend_from_slice(&divider_packets);
    packets.sort_unstable_by(value_cmp);
    divider_packets
        .iter()
        .map(|divider_packet| {
            packets
                .binary_search_by(|seek| value_cmp(seek, divider_packet))
                .unwrap()
                + 1
        })
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

    const INPUT: &str = include_str!("../../../test_data/year_2022/day_13.txt");

    #[test_case(EXAMPLE, 13)]
    #[test_case(INPUT, 5506)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 140)]
    #[test_case(INPUT, 21756)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
