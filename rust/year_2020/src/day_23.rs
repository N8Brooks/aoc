use std::iter::successors;

use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    game(input, 100, 9).take(8).fold(0, |n, cup| n * 10 + cup)
}

pub fn part_2(input: &str) -> usize {
    game(input, 10_000_000, 1_000_000).take(2).product()
}

fn game(input: &str, moves: usize, last: usize) -> impl Iterator<Item = usize> {
    let cups: [_; 9] = input
        .bytes()
        .map(|b| (b - b'1').into())
        .collect_array()
        .unwrap();

    let mut next: Vec<_> = (1..last).chain([cups[0]]).collect();

    for &[i, j] in cups.array_windows() {
        next[i] = j;
    }
    next[*cups.last().unwrap()] = if last == cups.len() {
        cups[0]
    } else {
        cups.len()
    };

    let mut cur = cups[0];

    for _ in 0..moves {
        let a = next[cur];
        let b = next[a];
        let c = next[b];
        next[cur] = next[c];

        let mut dest = cur;
        while {
            dest = dest.checked_sub(1).unwrap_or(last - 1);
            a == dest || b == dest || c == dest
        } {}

        (next[dest], next[c]) = (a, next[dest]);
        cur = next[cur];
    }

    successors(Some(next[0]), move |&cup| {
        (next[cup] != 0).then_some(next[cup])
    })
    .map(|cup| cup + 1)
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_23.txt");

    #[test_case("389125467" => 67384529)]
    #[test_case(INPUT => 32897654)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case("389125467" => 149245887792)]
    #[test_case(INPUT => 186715244496)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
