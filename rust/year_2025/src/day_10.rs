use std::{array, iter::successors, mem};

use hashbrown::HashMap;
use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    parse_input_1(input)
        .map(|(n, target, transitions)| {
            let mut new = vec![true; 1 << n];
            new[0] = false;
            successors(Some(vec![0]), |states| {
                let states: Vec<_> = states
                    .iter()
                    .cartesian_product(&transitions)
                    .map(|(state, transition)| state ^ transition)
                    .filter(|&state| mem::replace(&mut new[state], false))
                    .collect();
                (!states.is_empty()).then_some(states)
            })
            .enumerate()
            .find_map(|(steps, states)| states.contains(&target).then_some(steps))
            .expect("no solution found")
        })
        .sum()
}

fn parse_input_1(input: &str) -> impl Iterator<Item = (usize, usize, Vec<usize>)> {
    input.lines().map(|line| {
        let (indicator, rest) = line.split_once(' ').unwrap();
        let (buttons, _) = rest.rsplit_once(' ').unwrap();

        let lights = indicator.strip_circumfix('[', ']').unwrap();
        let n = lights.len();
        let target = lights
            .bytes()
            .rev()
            .map(|b| (b == b'#') as usize)
            .fold(0, |acc, b| (acc << 1) | b);

        let buttons = buttons
            .split(' ')
            .map(|button| {
                button
                    .strip_circumfix('(', ')')
                    .unwrap()
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .fold(0, |acc, i: u32| acc | (1 << i))
            })
            .collect();

        (n, target, buttons)
    })
}

pub fn part_2(input: &str) -> usize {
    parse_input_2(input)
        .map(|(target, buttons)| solve_machine(&target, &buttons).unwrap())
        .sum()
}

fn parse_input_2(input: &str) -> impl Iterator<Item = (Vec<u16>, Vec<u16>)> {
    input.lines().map(|line| {
        let (_, rest) = line.split_once(' ').unwrap();
        let (buttons, joltage) = rest.rsplit_once(' ').unwrap();

        let buttons = buttons
            .split(' ')
            .map(|button| {
                button
                    .strip_circumfix('(', ')')
                    .unwrap()
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .fold(0, |acc, i: u32| acc | (1 << i))
            })
            .collect();

        let target = joltage
            .strip_circumfix('{', '}')
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        (target, buttons)
    })
}

/// Parity-halving DP:
/// min sum x_j s.t. for each i: sum_{j with mask_j has i} x_j = target[i], x_j >= 0 integer.
/// n <= 10 dims, m <= 13 buttons.
fn solve_machine(target: &[u16], buttons: &[u16]) -> Option<usize> {
    const N: usize = 10;
    let (n, m) = (target.len(), buttons.len());
    let target = array::from_fn(|i| target.get(i).copied().unwrap_or(0));

    // Precompute subset contributions
    let size = 1 << m;
    let mut parity = vec![0; size]; // parity bitmask over dims
    let mut cnt = vec![[0; N]; size]; // counts per dim (only first n used)

    for s in 1..size {
        let j = s.trailing_zeros() as usize; // which button was added
        let prev = s & (s - 1);

        parity[s] = parity[prev] ^ buttons[j];
        cnt[s] = cnt[prev];

        let mut bm = buttons[j];
        while bm != 0 {
            let i = bm.trailing_zeros() as usize;
            bm &= bm - 1;
            cnt[s][i] += 1;
        }
    }

    let mut by_parity = vec![Vec::new(); 1 << n];
    for (s, &p) in parity.iter().enumerate() {
        by_parity[p as usize].push(s as u16);
    }

    fn rec(
        rem: [u16; N],
        n: usize,
        by_parity: &[Vec<u16>],
        cnt: &[[u16; N]],
        memo: &mut HashMap<u128, Option<usize>>,
    ) -> Option<usize> {
        if (0..n).all(|i| rem[i] == 0) {
            return Some(0);
        }

        let key = rem
            .iter()
            .take(n)
            .fold(0, |acc, &x| (acc << 9) | (x as u128));
        if let Some(&res) = memo.get(&key) {
            return res;
        }

        let want = rem
            .iter()
            .take(n)
            .rev()
            .fold(0, |acc, &x| (acc << 1) | (x & 1));

        let res = by_parity[want as usize]
            .iter()
            .filter_map(|&s| {
                let s = s as usize;
                if rem.into_iter().zip(cnt[s]).any(|(r, c)| c > r) {
                    return None;
                }
                let next = rem
                    .into_iter()
                    .zip(cnt[s])
                    .map(|(r, c)| (r - c) / 2)
                    .collect_array()
                    .unwrap();

                rec(next, n, by_parity, cnt, memo).map(|sub| s.count_ones() as usize + 2 * sub)
            })
            .min();

        memo.insert(key, res);
        res
    }

    rec(target, n, &by_parity, &cnt, &mut HashMap::new())
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_10.txt");

    const EXAMPLE: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test_case(EXAMPLE => 7)]
    #[test_case(INPUT => 452)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 33)]
    #[test_case(INPUT => 17424)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
