use std::{array, iter::repeat_n, ops::BitXor};

pub fn part_1(input: &str) -> u16 {
    solve_1::<256>(input)
}

fn solve_1<const N: usize>(input: &str) -> u16 {
    let skips = input.split(',').map(|s| s.parse().unwrap());
    let list: [_; N] = knot_hash(skips);
    list[0] as u16 * list[1] as u16
}

pub fn part_2(input: &str) -> String {
    const EXTRA: [u8; 5] = [17, 31, 73, 47, 23];
    let skips = repeat_n(input.bytes().chain(EXTRA).map(usize::from), 64).flatten();
    let list: [_; 256] = knot_hash(skips);
    let chunks: [[_; 16]; _] = list.as_chunks().0.try_into().unwrap();
    let bytes = chunks.map(|chunk| chunk.into_iter().fold(0, BitXor::bitxor));
    let hash = u128::from_be_bytes(bytes);
    format!("{hash:032x}")
}

fn knot_hash<const N: usize>(skips: impl Iterator<Item = usize>) -> [u8; N] {
    let list = array::from_fn(|i| i.try_into().unwrap());
    let (mut list, r) = skips
        .enumerate()
        .fold((list, 0), |(mut list, r), (skip, len)| {
            list[..len].reverse();
            list.rotate_left((len + skip) % N);
            (list, (r + len + skip) % N)
        });
    list.rotate_right(r);
    list
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_10.txt");

    #[test_case(INPUT => 4480)]
    fn part_1(input: &str) -> u16 {
        super::part_1(input)
    }

    #[test_case("3,4,1,5" => 12)]
    fn solve_1(input: &str) -> u16 {
        super::solve_1::<5>(input)
    }

    #[test_case("" => "a2582a3a0e66e6e86e3812dcb672a272")]
    #[test_case("AoC 2017" => "33efeb34ea91902bb2f59c9920caa6cd")]
    #[test_case("1,2,3" => "3efbe78a8d82f29979031a4aa0b16a9d")]
    #[test_case("1,2,4" => "63960835bcdc130f0b66d7ff4f6a5a8e")]
    #[test_case(INPUT => "c500ffe015c83b60fad2e4b7d59dabc4")]
    fn part_2(input: &str) -> String {
        super::part_2(input)
    }
}
