use std::iter::{repeat, repeat_n, repeat_with};

use itertools::Itertools;

pub fn part_1(input: &str) -> usize {
    let mut disk = (0..)
        .map(Some)
        .zip(repeat(None))
        .flat_map(|(a, b)| [a, b])
        .zip(input.bytes())
        .flat_map(|(id, count)| repeat_n(id, (count - b'0') as usize))
        .collect_vec()
        .into_iter();
    repeat(())
        .map_while(|_| {
            disk.next().and_then(|next_block| {
                let last_file = || {
                    repeat_with(|| disk.next_back())
                        .while_some()
                        .flatten()
                        .next()
                };
                next_block.or_else(last_file)
            })
        })
        .enumerate()
        .map(|(i, id)| i * id)
        .sum()
}

pub fn part_2(input: &str) -> usize {
    let (mut files, mut spaces): (Vec<_>, Vec<_>) = input
        .bytes()
        .chain([b'0']) // Sentinel space for tuples + unzip
        .scan(0, |index_2, count| {
            let size = (count - b'0') as usize;
            let index_1 = *index_2;
            *index_2 += size;
            Some((index_1, size))
        })
        .tuples()
        .unzip();
    for (i, (file_index, file_size)) in files.iter_mut().enumerate().rev() {
        if let Some((space_index, space_size)) = spaces
            .iter_mut()
            .take(i)
            .find(|(_, space_size)| space_size >= file_size)
        {
            *file_index = *space_index;
            *space_index += *file_size;
            *space_size -= *file_size;
        }
    }
    files
        .into_iter()
        .enumerate()
        .map(|(id, (index, size))| {
            let indexes_sum = (index + index + size - 1) * size / 2;
            indexes_sum * id
        })
        .sum()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../../../test_data/year_2024/day_09.txt");

    const EXAMPLE: &str = "2333133121414131402";

    #[test_case(EXAMPLE, 1928)]
    #[test_case(INPUT, 6279058075753)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 2858)]
    #[test_case(INPUT, 6301361958738)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
