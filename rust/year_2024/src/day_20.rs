use itertools::Itertools;

pub fn part_1(input: &str, save: usize) -> usize {
    count_cheats(input, 2, save)
}

pub fn part_2(input: &str, save: usize) -> usize {
    count_cheats(input, 20, save)
}

fn count_cheats(input: &str, range: isize, save: usize) -> usize {
    let map = input.lines().map(|line| line.as_bytes()).collect_vec();
    let (m, n) = (map.len(), map[0].len());
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
            .exactly_one()
            .unwrap();
        track[i2][j2] = Some(dist);
        dist += 1;
    }

    track
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(j, &dist)| dist.map(|dist| (i, j, dist)))
        })
        .flat_map(|(i1, j1, dist_1)| {
            (-range..=range).flat_map(move |di| {
                let dj = range - di.abs();
                (-dj..=dj).filter_map(move |dj| {
                    i1.checked_add_signed(di)
                        .zip(j1.checked_add_signed(dj))
                        .filter(|&(i2, j2)| i2 < m && j2 < n)
                        .map(|(i2, j2)| (dist_1 + i2.abs_diff(i1) + j2.abs_diff(j1), (i2, j2)))
                })
            })
        })
        .filter_map(|(dist_1, (i2, j2))| {
            track[i2][j2].and_then(|dist_2| dist_2.checked_sub(dist_1))
        })
        .filter(|&cheat| cheat + 1 >= save)
        .count()
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

    #[test_case(EXAMPLE, 10, 10)]
    #[test_case(INPUT, 100, 1415)]
    fn part_1(input: &str, save: usize, expected: usize) {
        assert_eq!(super::part_1(input, save), expected);
    }

    #[test_case(EXAMPLE, 70, 41)]
    #[test_case(INPUT, 100, 1022577)]
    fn part_2(input: &str, save: usize, expected: usize) {
        assert_eq!(super::part_2(input, save), expected);
    }
}
