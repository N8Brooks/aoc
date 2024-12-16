use std::iter::once;

use itertools::Itertools;

pub fn part_1(input: &str) -> usize {
    let mat = input.lines().map(|l| l.as_bytes()).collect_vec();
    let n = mat.len(); // Assume square matrix
    let mut id = 0;
    let mut ids = vec![vec![None; n]; n];
    mat.iter()
        .enumerate()
        .flat_map(|(i, l)| {
            l.iter()
                .enumerate()
                .filter(|(_, &c)| c != b'.')
                .map(move |(j, &c)| (i, j, c))
        })
        .filter_map(|(i, j, c)| {
            if ids[i][j].is_some() {
                return None;
            }
            ids[i][j] = Some(id);
            let mut stack = vec![(i, j)];
            let mut area = 1;
            let mut perimeter = 0;
            while let Some((i1, j1)) = stack.pop() {
                for coord in [
                    i1.checked_sub(1).map(|i2| (i2, j1)),
                    (i1 + 1 < n).then_some((i1 + 1, j1)),
                    j1.checked_sub(1).map(|j2| (i1, j2)),
                    (j1 + 1 < n).then_some((i1, j1 + 1)),
                ] {
                    if let Some((i2, j2)) = coord {
                        if mat[i2][j2] != c {
                            perimeter += 1;
                        } else if ids[i2][j2].is_none() {
                            ids[i2][j2] = Some(id);
                            area += 1;
                            stack.push((i2, j2));
                        }
                    } else {
                        perimeter += 1;
                    }
                }
            }
            id += 1;
            Some(area * perimeter)
        })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    let mat = input.lines().map(|l| l.as_bytes()).collect_vec();
    let n = mat.len(); // Assume square matrix
    let mut id = 0;
    let mut rows = vec![vec![None; n]; n];
    let mut areas = vec![];
    mat.iter()
        .enumerate()
        .flat_map(|(i, l)| {
            l.iter()
                .enumerate()
                .filter(|(_, &c)| c != b'.')
                .map(move |(j, &c)| (i, j, c))
        })
        .for_each(|(i, j, c)| {
            if rows[i][j].is_some() {
                return;
            }
            rows[i][j] = Some(id);
            let mut stack = vec![(i, j)];
            let mut area = 1;
            while let Some((i1, j1)) = stack.pop() {
                let new = [
                    i1.checked_sub(1).map(|i2| (i2, j1)),
                    (i1 + 1 < n).then_some((i1 + 1, j1)),
                    j1.checked_sub(1).map(|j2| (i1, j2)),
                    (j1 + 1 < n).then_some((i1, j1 + 1)),
                ]
                .into_iter()
                .flatten()
                .filter(|&(i, j)| mat[i][j] == c && rows[i][j].is_none())
                .collect_vec();
                new.iter().for_each(|&(i2, j2)| rows[i2][j2] = Some(id));
                area += new.len();
                stack.extend(new);
            }
            areas.push(area);
            id += 1;
        });

    let mut sides = vec![0; id];
    let cols = (0..n)
        .map(|j| rows.iter().map(|row| row[j]).collect_vec())
        .collect_vec();
    let blank = vec![None; n];
    for ids in [rows, cols] {
        for (row_1, row_2) in once(&blank).chain(&ids).tuple_windows() {
            row_1
                .iter()
                .zip(row_2)
                .map(|(id_1, id_2)| (id_1 != id_2).then(|| id_2.unwrap()))
                .chunk_by(|&id_2| id_2)
                .into_iter()
                .filter_map(|(id_2, _)| id_2)
                .for_each(|id_2| sides[id_2] += 1);
        }
        for (row_1, row_2) in ids.iter().chain(once(&blank)).tuple_windows() {
            row_1
                .iter()
                .zip(row_2)
                .map(|(id_1, id_2)| (id_1 != id_2).then(|| id_1.unwrap()))
                .chunk_by(|&id_1| id_1)
                .into_iter()
                .filter_map(|(id_1, _)| id_1)
                .for_each(|id_1| sides[id_1] += 1);
        }
    }

    areas.into_iter().zip(sides).map(|(a, s)| a * s).sum()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_12.txt");

    const EXAMPLE_1: &str = "AAAA
BBCD
BBCC
EEEC";

    const EXAMPLE_2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    const EXAMPLE_3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test_case(EXAMPLE_1, 140)]
    #[test_case(EXAMPLE_2, 772)]
    #[test_case(EXAMPLE_3, 1930)]
    #[test_case(INPUT, 1550156)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    const EXAMPLE_4: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

    const EXAMPLE_5: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

    #[test_case(EXAMPLE_1, 80)]
    #[test_case(EXAMPLE_2, 436)]
    #[test_case(EXAMPLE_3, 1206)]
    #[test_case(EXAMPLE_4, 236)]
    #[test_case(EXAMPLE_5, 368)]
    #[test_case(INPUT, 946084)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
