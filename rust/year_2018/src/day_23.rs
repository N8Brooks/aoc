use std::{cmp::Reverse, collections::BinaryHeap, iter::once};

use itertools::{Either::*, Itertools as _};

pub fn part_1(input: &str) -> usize {
    let bots = parse_bots(input);
    let &(a, r) = bots.iter().max_by_key(|(_, r)| *r).unwrap();
    bots.into_iter()
        .filter(|&(b, _)| {
            let dist: usize = a.iter().zip(b).map(|(a, b)| a.abs_diff(b)).sum();
            dist <= r
        })
        .count()
}

pub fn part_2(input: &str) -> usize {
    let bots = parse_bots(input);
    let bounds_0 = [0, 1, 2].map(|i| {
        bots.iter()
            .map(|(pos, _)| pos[i])
            .minmax()
            .into_option()
            .unwrap()
    });
    let mut heap = BinaryHeap::from([(bots.len(), Reverse(0), bounds_0)]);
    while let Some((_, Reverse(dist), bounds)) = heap.pop() {
        if bounds.iter().all(|(lo, hi)| lo == hi) {
            return dist;
        }

        bounds
            .into_iter()
            .map(|(lo, hi)| {
                if lo == hi {
                    Left(once((lo, hi)))
                } else {
                    let mi = lo.midpoint(hi);
                    Right([(lo, mi), (mi + 1, hi)].into_iter())
                }
            })
            .multi_cartesian_product()
            .map(|bounds| {
                let bounds: [_; 3] = bounds.try_into().unwrap();
                let count = bots
                    .iter()
                    .filter(|(pos, r)| {
                        let dist: usize = pos
                            .iter()
                            .zip(&bounds)
                            .map(|(p, (lo, hi))| p.clamp(lo, hi).abs_diff(*p))
                            .sum();
                        dist <= *r
                    })
                    .count();
                let dist = bounds
                    .iter()
                    .map(|&(lo, hi)| 0.clamp(lo, hi).unsigned_abs())
                    .sum();
                (count, Reverse(dist), bounds)
            })
            .collect_into(&mut heap);
    }
    panic!("no solution found");
}

fn parse_bots(input: &str) -> Vec<([isize; 3], usize)> {
    input
        .lines()
        .map(|line| {
            let (pos, r) = line.split_once(", ").unwrap();
            let pos = pos.strip_circumfix("pos=<", ">").unwrap();
            let pos = pos
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect_array()
                .unwrap();
            let r = r.strip_prefix("r=").unwrap().parse().unwrap();
            (pos, r)
        })
        .collect()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_23.txt");

    const EXAMPLE_1: &str = "pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1";

    #[test_case(EXAMPLE_1 => 7)]
    #[test_case(INPUT => 410)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    const EXAMPLE_2: &str = "pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5";

    #[test_case(EXAMPLE_2 => 36)]
    #[test_case(INPUT => 119188816)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
