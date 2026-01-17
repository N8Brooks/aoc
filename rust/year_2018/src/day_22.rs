use std::{cmp::Reverse, collections::BinaryHeap, iter::successors, mem};

pub fn part_1(input: &str) -> usize {
    let (depth, target) = parse_input(input);
    parse_map(depth, target, target)
        .into_iter()
        .flatten()
        .map(|region| region as usize)
        .sum()
}

pub fn part_2(input: &str) -> usize {
    use Gear::Torch;
    let (depth, target @ (n, m)) = parse_input(input);
    let map = parse_map(depth, target, (n + 50, m + 50));

    let mut heap = BinaryHeap::from([(Reverse(0), (0, 0), Torch)]);
    let mut seen = vec![vec![[false; 3]; map[0].len()]; map.len()];

    while let Some((Reverse(dist), pos @ (i, j), gear)) = heap.pop() {
        if pos == (n, m) && gear == Torch {
            return dist;
        }
        if mem::replace(&mut seen[i][j][gear as usize], true) {
            continue;
        }

        let region = map[i][j];
        let gear_2: Gear = (3 - region as u8 - gear as u8).into();
        if !seen[i][j][gear_2 as usize] {
            heap.push((Reverse(dist + 7), pos, gear_2));
        }

        [
            i.checked_sub(1).map(|i| (i, j)),
            (i + 1 < map.len()).then_some((i + 1, j)),
            j.checked_sub(1).map(|j| (i, j)),
            (j + 1 < map[0].len()).then_some((i, j + 1)),
        ]
        .into_iter()
        .flatten()
        .filter(|&(i, j)| !seen[i][j][gear as usize] && (gear as u8 != map[i][j] as u8))
        .map(|pos| (Reverse(dist + 1), pos, gear))
        .collect_into(&mut heap);
    }
    panic!("no path found");
}

fn parse_input(input: &str) -> (usize, (usize, usize)) {
    let (depth, target) = input.split_once('\n').unwrap();
    let depth = depth.strip_prefix("depth: ").unwrap().parse().unwrap();
    let (m, n) = target
        .strip_prefix("target: ")
        .unwrap()
        .split_once(',')
        .unwrap();
    let n = n.parse().unwrap();
    let m = m.parse().unwrap();
    (depth, (n, m))
}

fn parse_map(depth: usize, target: (usize, usize), (n, m): (usize, usize)) -> Vec<Vec<Region>> {
    const MOD: usize = 20_183;
    let row_0: Vec<_> = (0..=m).map(|x| (x * 16_807 + depth) % MOD).collect();
    let mut i = 0;
    successors(Some(row_0), |row| {
        i += 1;
        let left_0 = (i * 48_271 + depth) % MOD;
        let mut aboves = row.iter().skip(1);
        let mut j = 0;
        let row: Vec<_> = successors(Some(left_0), |&left| {
            j += 1;
            let above = aboves.next()?;
            Some((left * above * ((i, j) != target) as usize + depth) % MOD)
        })
        .take(m + 1)
        .collect();
        Some(row)
    })
    .take(n + 1)
    .map(|row| {
        row.into_iter()
            .map(|level| (level % 3).into())
            .collect::<Vec<_>>()
    })
    .collect()
}

#[repr(u8)]
#[derive(Copy, Clone)]
enum Region {
    Rocky = 0,
    Wet = 1,
    Narrow = 2,
}

impl From<usize> for Region {
    fn from(region: usize) -> Self {
        use Region::*;
        match region {
            0 => Rocky,
            1 => Wet,
            2 => Narrow,
            _ => panic!("invalid region {region}"),
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Gear {
    Neither = 0,
    Torch = 1,
    Climbing = 2,
}

impl From<u8> for Gear {
    fn from(gear: u8) -> Self {
        use Gear::*;
        match gear {
            0 => Neither,
            1 => Torch,
            2 => Climbing,
            _ => panic!("invalid gear {gear}"),
        }
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_22.txt");

    const EXAMPLE: &str = "depth: 510\ntarget: 10,10";

    #[test_case(EXAMPLE => 114)]
    #[test_case(INPUT => 7901)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 45)]
    #[test_case(INPUT => 1087)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
