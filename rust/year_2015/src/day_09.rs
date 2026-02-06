use std::cmp::{max, min};

pub fn part_1(input: &str) -> usize {
    let dists = parse_dists(input);
    tsp(&dists, min)
}

pub fn part_2(input: &str) -> usize {
    let dists = parse_dists(input);
    tsp(&dists, max)
}

fn parse_dists(input: &str) -> Vec<Vec<usize>> {
    let inf = usize::MAX;
    let mut cities = vec![];
    let mut dists: Vec<Vec<_>> = vec![];
    for line in input.lines() {
        let (left, rest) = line.split_once(" to ").unwrap();
        let (right, dist) = rest.split_once(" = ").unwrap();
        let dist: usize = dist.parse().unwrap();
        let i = if let Some(i) = cities.iter().position(|&c| c == left) {
            i
        } else {
            cities.push(left);
            dists.iter_mut().for_each(|row| row.push(inf));
            dists.push(vec![inf; cities.len()]);
            cities.len() - 1
        };
        let j = if let Some(j) = cities.iter().position(|&c| c == right) {
            j
        } else {
            cities.push(right);
            dists.iter_mut().for_each(|row| row.push(inf));
            dists.push(vec![inf; cities.len()]);
            cities.len() - 1
        };
        dists[i][j] = dist;
        dists[j][i] = dist;
    }
    dists
}

fn tsp(dists: &[Vec<usize>], pick: impl Fn(usize, usize) -> usize + Copy) -> usize {
    fn tsp(
        dp: &mut [Vec<Option<usize>>],
        dists: &[Vec<usize>],
        mask: usize,
        pos: usize,
        pick: impl Fn(usize, usize) -> usize + Copy,
    ) -> usize {
        if mask == (1 << dists.len()) - 1 {
            return 0;
        }
        if let Some(res) = dp[mask][pos] {
            return res;
        }
        let res = dists[pos]
            .iter()
            .enumerate()
            .filter(|(i, _)| mask & (1 << i) == 0)
            .map(|(i, &dist)| dist + tsp(dp, dists, mask | (1 << i), i, pick))
            .reduce(pick)
            .unwrap();
        dp[mask][pos] = Some(res);
        res
    }

    let n = dists.len();
    let mut dp = vec![vec![None; n]; 1 << n];
    (0..n)
        .map(|i| tsp(&mut dp, dists, 1 << i, i, pick))
        .reduce(pick)
        .unwrap()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_09.txt");

    const EXAMPLE: &str = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";

    #[test_case(EXAMPLE => 605)]
    #[test_case(INPUT => 117)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 982)]
    #[test_case(INPUT => 909)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
