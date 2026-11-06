use std::{array, iter::repeat_n, mem, ops::BitXor};

use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    (0..128)
        .map(|i| -> usize {
            let s = format!("{input}-{i}");
            let hash = knot_hash(&s);
            hash.count_ones().try_into().unwrap()
        })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    let mut map: [[_; 128]; 128] = (0..128)
        .map(|i| {
            let s = format!("{input}-{i}");
            let hash = knot_hash(&s);
            array::from_fn(|j| (hash >> j) & 1 == 1)
        })
        .collect_array()
        .unwrap();
    (0..128)
        .flat_map(|i| (0..128).map(move |j| (i, j)))
        .filter(|&(i, j)| {
            if !mem::replace(&mut map[i][j], false) {
                return false;
            }
            let mut stack = vec![(i, j)];
            while let Some((i, j)) = stack.pop() {
                [
                    i.checked_sub(1).map(|i| (i, j)),
                    (i + 1 < 128).then(|| (i + 1, j)),
                    j.checked_sub(1).map(|j| (i, j)),
                    (j + 1 < 128).then(|| (i, j + 1)),
                ]
                .into_iter()
                .flatten()
                .filter(|&(i, j)| mem::replace(&mut map[i][j], false))
                .collect_into(&mut stack);
            }
            true
        })
        .count()
}

pub fn knot_hash(input: &str) -> u128 {
    const EXTRA: [u8; 5] = [17, 31, 73, 47, 23];
    let list: [u8; 256] = array::from_fn(|i| i.try_into().unwrap());
    let (mut list, r) = repeat_n(input.bytes().chain(EXTRA).map(usize::from), 64)
        .flatten()
        .enumerate()
        .fold((list, 0), |(mut list, r), (skip, len)| {
            list[..len].reverse();
            list.rotate_left((len + skip) % 256);
            (list, (r + len + skip) % 256)
        });
    list.rotate_right(r);
    let chunks: [[_; 16]; _] = list.as_chunks().0.try_into().unwrap();
    let bytes = chunks.map(|chunk| chunk.into_iter().fold(0, BitXor::bitxor));
    u128::from_be_bytes(bytes)
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "flqrgnkx";

    const INPUT: &str = include_str!("../test_data/day_14.txt");

    #[test_case(EXAMPLE => 8108)]
    #[test_case(INPUT => 8106)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 1242)]
    #[test_case(INPUT => 1164)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
