use itertools::{Itertools, iproduct};

pub fn part_1<const N: usize>(input: &str, steps: usize) -> usize {
    let grid_0: [[_; N]; N] = input
        .lines()
        .map(|line| line.bytes().map(|b| b == b'#').collect_array().unwrap())
        .collect_array()
        .unwrap();
    (0..steps)
        .fold(grid_0, |grid, _| step(grid))
        .into_iter()
        .flatten()
        .filter(|&cell| cell)
        .count()
}

pub fn part_2<const N: usize>(input: &str, steps: usize) -> usize {
    let mut grid_0: [[_; N]; N] = input
        .lines()
        .map(|line| line.bytes().map(|b| b == b'#').collect_array().unwrap())
        .collect_array()
        .unwrap();
    grid_0[0][0] = true;
    grid_0[0][N - 1] = true;
    grid_0[N - 1][0] = true;
    grid_0[N - 1][N - 1] = true;
    (0..steps)
        .fold(grid_0, |grid, _| {
            let mut grid = step(grid);
            grid[0][0] = true;
            grid[0][N - 1] = true;
            grid[N - 1][0] = true;
            grid[N - 1][N - 1] = true;
            grid
        })
        .into_iter()
        .flatten()
        .filter(|&cell| cell)
        .count()
}

fn step<const N: usize>(grid: [[bool; N]; N]) -> [[bool; N]; N] {
    grid.into_iter()
        .enumerate()
        .map(|(i, row)| {
            row.into_iter()
                .enumerate()
                .map(|(j, cell)| {
                    let neighbors = iproduct!(
                        i.saturating_sub(1)..=(i + 1).min(N - 1),
                        j.saturating_sub(1)..=(j + 1).min(N - 1)
                    )
                    .filter(|&(x, y)| (x, y) != (i, j))
                    .filter(|&(x, y)| grid[x][y])
                    .count();
                    matches!((cell, neighbors), (true, 2 | 3) | (false, 3))
                })
                .collect_array()
                .unwrap()
        })
        .collect_array()
        .unwrap()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_18.txt");

    const EXAMPLE: &str = ".#.#.#
...##.
#....#
..#...
#.#..#
####..";

    #[test_case(EXAMPLE, 5 => 4)]
    fn part_1_example(input: &str, steps: usize) -> usize {
        super::part_1::<6>(input, steps)
    }
    #[test_case(INPUT, 100 => 1061)]
    fn part_1(input: &str, steps: usize) -> usize {
        super::part_1::<100>(input, steps)
    }

    #[test_case(EXAMPLE, 5 => 17)]
    fn part_2_example(input: &str, steps: usize) -> usize {
        super::part_2::<6>(input, steps)
    }

    #[test_case(INPUT, 100 => 1006)]
    fn part_2(input: &str, steps: usize) -> usize {
        super::part_2::<100>(input, steps)
    }
}
