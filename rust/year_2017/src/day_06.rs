use std::mem;

use num::Integer as _;

pub fn part_1(input: &str) -> usize {
    let (mu, lam) = floyd(input);
    mu + lam
}

pub fn part_2(input: &str) -> usize {
    floyd(input).1
}

fn floyd(input: &str) -> (usize, usize) {
    let x0 = parse_blocks(input);
    let mut tortoise = x0.clone();
    f(&mut tortoise);
    let mut hare = tortoise.clone();
    f(&mut hare);
    while tortoise != hare {
        f(&mut tortoise);
        f(&mut hare);
        f(&mut hare);
    }

    let mut mu = 0;
    tortoise = x0;
    while tortoise != hare {
        f(&mut tortoise);
        f(&mut hare);
        mu += 1;
    }

    let mut lam = 1;
    let mut hare = tortoise.clone();
    f(&mut hare);
    while tortoise != hare {
        f(&mut hare);
        lam += 1;
    }

    (mu, lam)
}

fn parse_blocks(input: &str) -> Vec<usize> {
    input
        .split('\t')
        .map(|line| line.parse().unwrap())
        .collect()
}

#[inline]
fn f(blocks: &mut [usize]) {
    let n = blocks.len();
    let i = (0..n).rev().max_by_key(|&i| blocks[i]).unwrap();
    let max = mem::take(&mut blocks[i]);
    let (q, r) = max.div_rem(&n);
    let (before, after) = blocks.split_at_mut(i + 1);
    for (offset, block) in after.iter_mut().chain(before).enumerate() {
        *block += q + usize::from(offset < r);
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_06.txt");

    const EXAMPLE: &str = "0	2	7	0";

    #[test_case(EXAMPLE => 5)]
    #[test_case(INPUT => 12841)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 4)]
    #[test_case(INPUT => 8038)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
