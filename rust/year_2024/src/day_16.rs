use std::{cmp::Reverse, collections::BinaryHeap};

use hashbrown::{HashMap, HashSet};
use itertools::Itertools as _;
use num::Complex;

/// Represents a position and direction in the maze.
type Pose = (Complex<i64>, Complex<i64>);

pub fn part_1(input: &str) -> usize {
    let maze = input.lines().map(|line| line.as_bytes()).collect_vec();
    let start = (maze.len() as i64 - 2, 1); // assume bottom left position
    let end = (1, maze[0].len() as i64 - 2); // assume top right position
    let mut heap = BinaryHeap::from([(Reverse(0), start, (0, 1))]);
    let mut seen: HashSet<Pose> =
        HashSet::from([(Complex::new(start.0, start.1), Complex::new(0, 1))]);
    while heap.peek().unwrap().1 != end {
        let (dist_1, (i1, j1), (di1, dj1)) = heap.pop().unwrap();
        let pos_1 = Complex::new(i1, j1);
        let dir_1 = Complex::new(di1, dj1);
        for (delta, dir_2) in [
            (1, dir_1),
            (1001, dir_1 * Complex::i()),
            (1001, dir_1 * -Complex::i()),
        ] {
            let pos_2 = pos_1 + dir_2;
            let i2 = pos_2.re as usize;
            let j2 = pos_2.im as usize;
            if maze[i2][j2] != b'#' && seen.insert((pos_2, dir_2)) {
                heap.push((
                    Reverse(dist_1.0 + delta),
                    (pos_2.re, pos_2.im),
                    (dir_2.re, dir_2.im),
                ));
            }
        }
    }
    let (dist, _, _) = heap.peek().unwrap();
    dist.0
}

pub fn part_2(input: &str) -> usize {
    let maze = input.lines().map(|line| line.as_bytes()).collect_vec();
    let start = (maze.len() as i64 - 2, 1); // assume bottom left position
    let end = (1, maze[0].len() as i64 - 2); // assume top right position
    let mut heap = BinaryHeap::from([(Reverse(0), start, (0, 1))]);
    let mut dists: HashMap<Pose, usize> =
        HashMap::from([((Complex::new(start.0, start.1), Complex::new(0, 1)), 0)]);
    let mut prevs: HashMap<Pose, Vec<Pose>> = HashMap::from([(
        (Complex::new(start.0, start.1), Complex::new(0, 1)),
        Vec::new(),
    )]);
    while let Some((dist_1, (i1, j1), (di1, dj1))) = heap.pop() {
        let pos_1 = Complex::new(i1, j1);
        let dir_1 = Complex::new(di1, dj1);
        for (delta, dir_2) in [
            (1, dir_1),
            (1001, dir_1 * Complex::i()),
            (1001, dir_1 * -Complex::i()),
        ] {
            let pos_2 = pos_1 + dir_2;
            let i2 = pos_2.re as usize;
            let j2 = pos_2.im as usize;
            if maze[i2][j2] != b'#'
                && dists
                    .get(&(pos_2, dir_2))
                    .is_none_or(|&d| d >= dist_1.0 + delta)
            {
                dists.insert((pos_2, dir_2), dist_1.0 + delta);
                prevs
                    .entry((pos_2, dir_2))
                    .or_default()
                    .push((pos_1, dir_1));
                heap.push((
                    Reverse(dist_1.0 + delta),
                    (pos_2.re, pos_2.im),
                    (dir_2.re, dir_2.im),
                ));
            }
        }
    }
    let mut stack = [Complex::new(0, 1), Complex::new(-1, 0)]
        .into_iter()
        .map(|dir| (Complex::new(end.0, end.1), dir))
        .filter(|pose| prevs.contains_key(pose))
        .min_set_by_key(|&pose| dists[&pose])
        .into_iter()
        .collect_vec();
    let mut seen: HashSet<Pose> = HashSet::from_iter(stack.iter().copied());
    while let Some((pos, dir)) = stack.pop() {
        stack.extend(
            prevs[&(pos, dir)]
                .iter()
                .copied()
                .filter(|&pose| seen.insert(pose)),
        );
    }
    seen.into_iter()
        .map(|(pos, _)| pos)
        .collect::<HashSet<_>>()
        .len()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_16.txt");

    const EXAMPLE_1: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const EXAMPLE_2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test_case(EXAMPLE_1, 7036)]
    #[test_case(EXAMPLE_2, 11048)]
    #[test_case(INPUT, 98484)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE_1, 45)]
    #[test_case(EXAMPLE_2, 64)]
    #[test_case(INPUT, 531)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
