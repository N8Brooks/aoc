use std::{array, iter::successors};

use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    let grid0: [[_; 5]; 5] = input
        .lines()
        .map(|line| line.bytes().map(|b| b == b'#').next_chunk().unwrap())
        .next_chunk()
        .unwrap();
    let last = successors(Some(grid0), |&grid| {
        Some(array::from_fn(|i| {
            array::from_fn(|j| {
                let mut cnt = 0;
                if i > 0 && grid[i - 1][j] {
                    cnt += 1;
                }
                if i < 4 && grid[i + 1][j] {
                    cnt += 1;
                }
                if j > 0 && grid[i][j - 1] {
                    cnt += 1;
                }
                if j < 4 && grid[i][j + 1] {
                    cnt += 1;
                }
                cnt == 1 || (!grid[i][j] && cnt == 2)
            })
        }))
    })
    .duplicates()
    .next()
    .unwrap();
    last.into_iter()
        .flatten()
        .enumerate()
        .filter(|&(_, b)| b)
        .map(|(i, _)| 1 << i)
        .sum()
}

pub fn part_2(input: &str) -> usize {
    fun::<200>(input)
}

fn fun<const N: usize>(input: &str) -> usize
where
    [(); 2 * N + 1]:,
{
    let mut grids0 = [[[false; 5]; 5]; 2 * N + 1];
    grids0[N] = input
        .lines()
        .map(|line| line.bytes().map(|b| b == b'#').next_chunk().unwrap())
        .next_chunk()
        .unwrap();
    let last = (1..=N).fold(grids0, |grids, step| {
        array::from_fn(|k| {
            if k < N - step || k > N + step {
                return [[false; 5]; 5];
            }
            array::from_fn(|i| {
                array::from_fn(|j| {
                    if i == 2 && j == 2 {
                        return false;
                    }
                    let cnt = if i == 0 {
                        k.checked_sub(1).map_or(0, |k| grids[k][1][2] as usize)
                    } else if i == 3 && j == 2 {
                        grids
                            .get(k + 1)
                            .map_or(0, |g| g[4].into_iter().map(|b| b as usize).sum())
                    } else {
                        grids[k][i - 1][j] as usize
                    } + if i == 4 {
                        k.checked_sub(1).map_or(0, |k| grids[k][3][2] as usize)
                    } else if i == 1 && j == 2 {
                        grids
                            .get(k + 1)
                            .map_or(0, |g| g[0].into_iter().map(|b| b as usize).sum())
                    } else {
                        grids[k][i + 1][j] as usize
                    } + if j == 0 {
                        k.checked_sub(1).map_or(0, |k| grids[k][2][1] as usize)
                    } else if i == 2 && j == 3 {
                        grids
                            .get(k + 1)
                            .map_or(0, |g| g.iter().map(|row| row[4] as usize).sum())
                    } else {
                        grids[k][i][j - 1] as usize
                    } + if j == 4 {
                        k.checked_sub(1).map_or(0, |k| grids[k][2][3] as usize)
                    } else if i == 2 && j == 1 {
                        grids
                            .get(k + 1)
                            .map_or(0, |g| g.iter().map(|row| row[0] as usize).sum())
                    } else {
                        grids[k][i][j + 1] as usize
                    };
                    cnt == 1 || (!grids[k][i][j] && cnt == 2)
                })
            })
        })
    });
    last.into_iter().flatten().flatten().filter(|&b| b).count()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_24.txt");

    const EXAMPLE: &str = "\
....#
#..#.
#..##
..#..
#....";

    #[test_case(EXAMPLE => 2129920)]
    #[test_case(INPUT => 12129040)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 99)]
    fn fun(input: &str) -> usize {
        super::fun::<10>(input)
    }

    #[test_case(INPUT => 2109)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
