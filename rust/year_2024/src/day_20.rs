use std::iter::repeat;

use itertools::Itertools;

pub fn part_1(input: &str) -> usize {
    count_cheats(input, 2, 100)
}

pub fn part_2(input: &str) -> usize {
    count_cheats(input, 20, 100)
}

fn count_cheats(input: &str, range: isize, threshold: usize) -> usize {
    let track = &parse_track(input);
    let (m, n) = (track.len(), track[0].len());
    track
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(j, &dist)| dist.map(|dist| (i, j, dist)))
        })
        .flat_map(|(i1, j1, dist_1)| {
            (-range..=range)
                .flat_map(|di| {
                    let dj = range - di.abs();
                    repeat(di).zip(-dj..=dj)
                })
                .filter_map(move |(di, dj)| {
                    let i2 = i1.checked_add_signed(di).filter(|&i| i < m)?;
                    let j2 = j1.checked_add_signed(dj).filter(|&j| j < n)?;
                    let dist_2 = track[i2][j2]? + 1;
                    let cheat_dist = i2.abs_diff(i1) + j2.abs_diff(j1);
                    dist_2.checked_sub(dist_1 + cheat_dist)
                })
        })
        .filter(|&cheat| cheat >= threshold)
        .count()
}

fn parse_track(input: &str) -> Vec<Vec<Option<usize>>> {
    let map = input.lines().map(|line| line.as_bytes()).collect_vec();
    let start = map
        .iter()
        .enumerate()
        .find_map(|(i, row)| row.iter().position(|&b| b == b'S').map(|j| (i, j)))
        .unwrap();
    let end = map
        .iter()
        .enumerate()
        .find_map(|(i, row)| row.iter().position(|&b| b == b'E').map(|j| (i, j)))
        .unwrap();
    let (mut i1, mut j1) = (usize::MAX, usize::MAX);
    let (mut i2, mut j2) = start;
    let mut dist = 0;
    let mut track = vec![vec![None::<usize>; map[0].len()]; map.len()];
    track[i2][j2] = Some(0);
    while (i2, j2) != end {
        (i1, j1, i2, j2) = [(i2 + 1, j2), (i2 - 1, j2), (i2, j2 + 1), (i2, j2 - 1)]
            .into_iter()
            .filter(|&(i3, j3)| (i1, j1) != (i3, j3) && map[i3][j3] != b'#')
            .map(|(i3, j3)| (i2, j2, i3, j3))
            .next() // assume perfect maze
            .unwrap();
        track[i2][j2] = Some(dist);
        dist += 1;
    }
    track
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_20.txt");

    const EXAMPLE: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test_case(INPUT, 1415)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(INPUT, 1022577)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }

    #[test_case(EXAMPLE, 2, 10, 10)]
    #[test_case(EXAMPLE, 20, 70, 41)]
    fn count_cheats(input: &str, range: isize, threshold: usize, expected: usize) {
        assert_eq!(super::count_cheats(input, range, threshold), expected);
    }
}
