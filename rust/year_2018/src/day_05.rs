use std::cmp::Ordering;

use itertools::{FoldWhile, Itertools};

pub fn part_1(input: &str) -> usize {
    fully_react_polymer(input.as_bytes())
}

pub fn part_2(input: &str) -> usize {
    (b'a'..=b'z')
        .map(|lower| {
            let upper = lower.to_ascii_uppercase();
            let polymer = input
                .bytes()
                .filter(|&unit| unit != lower && unit != upper)
                .collect_vec();
            fully_react_polymer(&polymer)
        })
        .min()
        .unwrap()
}

fn fully_react_polymer(polymer: &[u8]) -> usize {
    (0..)
        .scan(Vec::from(polymer), |polymer, _| {
            *polymer = react_polymer(polymer);
            Some(polymer.len())
        })
        .fold_while(polymer.len(), |a, b| match a.cmp(&b) {
            Ordering::Greater => FoldWhile::Continue(b),
            Ordering::Equal => FoldWhile::Done(b),
            Ordering::Less => panic!("expected a not to be greater than b"),
        })
        .into_inner()
}

fn react_polymer(polymer: &[u8]) -> Vec<u8> {
    const UNIT_POLARITY_DIFF: u8 = b'a' - b'A';
    let mut polymer = polymer.iter();
    let first = polymer.next();
    polymer
        .chain(Some(&0)) // terminator to pop last `a`
        .scan(first, |state, b| {
            if let Some(a) = *state {
                if a.abs_diff(*b) == UNIT_POLARITY_DIFF {
                    *state = None; // `b` reacted
                    Some(None) // `a` reacted
                } else {
                    *state = Some(b); // `b` is pending reaction
                    Some(Some(a)) // `a` didn't react
                }
            } else {
                *state = Some(b); // `b` is pending reaction
                Some(None) // `a` reacted when it was `b`
            }
        })
        .flatten()
        .copied()
        .collect()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "dabAcCaCBAcCcaDA";

    const INPUT: &str = include_str!("../../../test_data/year_2018/day_05.txt");

    #[test_case("aA", 0; "reaction_example_1")]
    #[test_case("abBA", 0; "reaction_example_2")]
    #[test_case("abAB", 4; "reaction_example_3")]
    #[test_case("aabAAB", 6; "reaction_example_4")]
    #[test_case(EXAMPLE, 10; "example")]
    #[test_case(INPUT, 10886; "input")]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 4; "example")]
    #[test_case(INPUT, 4684; "input")]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
