use std::{cmp::Reverse, collections::BinaryHeap, iter::successors};

type Region = usize;

type Gear = usize;

pub fn part_1(input: &str) -> usize {
    let (depth, target) = parse_input(input);
    parse_map(depth, target, target).into_iter().flatten().sum()
}

pub fn part_2(input: &str) -> usize {
    const TORCH: Gear = 1;
    let (depth, target @ (i, j)) = parse_input(input);
    let (n, m) = (i + 50, j + 50);
    let map = parse_map(depth, target, (n, m));

    let mut heap = BinaryHeap::from([(Reverse(0), (0, 0), TORCH)]);
    let mut seen = vec![vec![0; map[0].len()]; map.len()];

    while let Some((Reverse(dist), pos @ (i, j), gear)) = heap.pop() {
        if pos == target && gear == TORCH {
            return dist;
        }
        if seen[i][j] & (1 << gear) != 0 {
            continue;
        }
        seen[i][j] |= 1 << gear;

        [
            i.checked_sub(1).map(|i| (i, j)),
            (i + 1 < n).then(|| (i + 1, j)),
            j.checked_sub(1).map(|j| (i, j)),
            (j + 1 < m).then(|| (i, j + 1)),
        ]
        .into_iter()
        .flatten()
        .filter(|&(i, j)| seen[i][j] & (1 << gear) == 0 && gear != map[i][j])
        .map(|pos| (Reverse(dist + 1), pos, gear))
        .collect_into(&mut heap);

        let region = map[i][j];
        let gear = 3 - region - gear;
        if seen[i][j] & (1 << gear) == 0 {
            heap.push((Reverse(dist + 7), pos, gear));
        }
    }
    panic!("no path found");
}

fn parse_input(input: &str) -> (usize, (usize, usize)) {
    let (depth, target) = input.split_once('\n').unwrap();
    let depth = depth.strip_prefix("depth: ").unwrap().parse().unwrap();
    let target = target.strip_prefix("target: ").unwrap();
    let (m, n) = target.split_once(',').unwrap();
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
    .map(|mut row| {
        row.iter_mut().for_each(|cell| *cell %= 3);
        row
    })
    .collect()
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
