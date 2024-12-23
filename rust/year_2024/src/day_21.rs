use std::{
    array,
    iter::once,
    sync::{Arc, LazyLock, Mutex},
};

use hashbrown::HashMap;
use itertools::repeat_n;

pub fn part_1(input: &str) -> usize {
    sum_complexities(input, 2)
}

pub fn part_2(input: &str) -> usize {
    sum_complexities(input, 25)
}

fn sum_complexities(input: &str, n: usize) -> usize {
    const NUM_KEYPAD: &[&[u8]] = &[b"789", b"456", b"123", b".0A"];
    input
        .lines()
        .map(|line| {
            let len = shortest_sequence_len(NUM_KEYPAD, line.bytes().collect(), n + 1);
            let num = line[0..3].parse::<usize>().unwrap(); // assume num position
            len * num
        })
        .sum()
}

fn shortest_sequence_len(keypad: &'static [&'static [u8]], code: Vec<u8>, n: usize) -> usize {
    const DIR_KEYPAD: &[&[u8]] = &[b".^A", b"<v>"];
    #[allow(clippy::type_complexity)]
    static MEMO: LazyLock<[Arc<Mutex<HashMap<Vec<u8>, usize>>>; 27]> =
        LazyLock::new(|| array::from_fn(|_| Arc::new(Mutex::new(HashMap::new()))));
    if n == 0 {
        return code.len();
    }
    if let Some(&len) = MEMO[n].lock().unwrap().get(&code) {
        return len;
    }
    let (mut i1, mut j1) = keypad
        .iter()
        .enumerate()
        .find_map(|(i, row)| row.iter().position(|&b| b == b'A').map(|j| (i, j)))
        .unwrap();
    let res = code
        .iter()
        .map(|c| {
            let (i2, j2) = keypad
                .iter()
                .enumerate()
                .find_map(|(i, row)| row.iter().position(|b| b == c).map(|j| (i, j)))
                .unwrap();
            let v = || {
                if i2 > i1 {
                    repeat_n(b'v', i2 - i1)
                } else {
                    repeat_n(b'^', i1 - i2)
                }
            };
            let h = || {
                if j2 > j1 {
                    repeat_n(b'>', j2 - j1)
                } else {
                    repeat_n(b'<', j1 - j2)
                }
            };
            let codes = [
                (keypad[i2][j1] != b'.').then(|| v().chain(h())),
                (keypad[i1][j2] != b'.').then(|| h().chain(v())),
            ];
            (i1, j1) = (i2, j2);
            codes
                .into_iter()
                .flatten()
                .map(|code| {
                    shortest_sequence_len(DIR_KEYPAD, code.chain(once(b'A')).collect(), n - 1)
                })
                .min()
                .unwrap()
        })
        .sum();
    MEMO[n].lock().unwrap().insert(code, res);
    res
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "029A
980A
179A
456A
379A";

    const INPUT: &str = include_str!("../test_data/day_21.txt");

    #[test_case(EXAMPLE, 126384)]
    #[test_case(INPUT, 179444)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 154115708116294)]
    #[test_case(INPUT, 223285811665866)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
