use hashbrown::HashSet;
use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    let mat = input.lines().map(|l| l.as_bytes()).collect_vec();
    let m = mat.len();
    let n = mat[0].len();
    mat.iter()
        .enumerate()
        .flat_map(|(i, l)| {
            l.iter()
                .enumerate()
                .filter(|&(_, &c)| c == b'0')
                .map(move |(j, _)| (i, j))
        })
        .map(|(i, j)| {
            let mut stack = vec![(i, j, b'0')];
            let mut nines = HashSet::new();
            while let Some((i1, j1, c1)) = stack.pop() {
                let new = [
                    i1.checked_sub(1).map(|i2| (i2, j1)),
                    (i1 + 1 < m).then(|| (i1 + 1, j1)),
                    j1.checked_sub(1).map(|j2| (i1, j2)),
                    (j1 + 1 < n).then(|| (i1, j1 + 1)),
                ]
                .into_iter()
                .flatten()
                .filter_map(|(i2, j2)| match mat[i2][j2] {
                    b'9' if c1 == b'8' => {
                        nines.insert((i2, j2));
                        None
                    }
                    c2 if c1 + 1 == c2 => Some((i2, j2, c2)),
                    _ => None,
                });
                stack.extend(new);
            }
            nines.len()
        })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    let mat = input.lines().map(|l| l.as_bytes()).collect_vec();
    let m = mat.len();
    let n = mat[0].len();
    mat.iter()
        .enumerate()
        .flat_map(|(i, l)| {
            l.iter()
                .enumerate()
                .filter(|&(_, &c)| c == b'0')
                .map(move |(j, _)| (i, j))
        })
        .map(|(i, j)| {
            let mut stack = vec![(Vec::new(), i, j, b'0')];
            let mut nines = HashSet::new();
            while let Some((path, i1, j1, c1)) = stack.pop() {
                let new = [
                    i1.checked_sub(1).map(|i2| (i2, j1)),
                    (i1 + 1 < m).then(|| (i1 + 1, j1)),
                    j1.checked_sub(1).map(|j2| (i1, j2)),
                    (j1 + 1 < n).then(|| (i1, j1 + 1)),
                ]
                .into_iter()
                .flatten()
                .filter_map(|(i2, j2)| match mat[i2][j2] {
                    b'9' if c1 == b'8' => {
                        let mut path = path.clone();
                        path.push((i2, j2));
                        nines.insert(path);
                        None
                    }
                    c2 if c1 + 1 == c2 => {
                        let mut path = path.clone();
                        path.push((i2, j2));
                        Some((path, i2, j2, c2))
                    }
                    _ => None,
                });
                stack.extend(new);
            }
            nines.len()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_10.txt");

    const EXAMPLE_1_1: &str = "0123
1234
8765
9876";

    const EXAMPLE_1_2: &str = "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9";

    const EXAMPLE_1_3: &str = "10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01";

    const EXAMPLE_1_4: &str = "..90..9
...1.98
...2..7
6543456
765.987
876....
987....";

    const EXAMPLE_LARGE: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test_case(EXAMPLE_1_1, 1)]
    #[test_case(EXAMPLE_1_2, 2)]
    #[test_case(EXAMPLE_1_4, 4)]
    #[test_case(EXAMPLE_1_3, 3)]
    #[test_case(EXAMPLE_LARGE, 36)]
    #[test_case(INPUT, 430)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    const EXAMPLE_2_3: &str = ".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....";

    const EXAMPLE_2_13: &str = "..90..9
...1.98
...2..7
6543456
765.987
876....
987....";

    const EXAMPLE_2_227: &str = "012345
123456
234567
345678
4.6789
56789.";

    #[test_case(EXAMPLE_2_3, 3)]
    #[test_case(EXAMPLE_2_13, 13)]
    #[test_case(EXAMPLE_2_227, 227)]
    #[test_case(EXAMPLE_LARGE, 81)]
    #[test_case(INPUT, 928)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
