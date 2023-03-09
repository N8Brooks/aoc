use std::{collections::HashSet, ops::Range};

use itertools::Itertools;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Face {
    Yz([isize; 3]),
    Xz([isize; 3]),
    Xy([isize; 3]),
}

pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .flat_map(|line| {
            let (x, y, z) = line
                .split(',')
                .map(|coordinate| coordinate.parse().unwrap())
                .collect_tuple()
                .unwrap();
            [
                Face::Yz([x, y, z]),
                Face::Yz([x + 1, y, z]),
                Face::Xz([x, y, z]),
                Face::Xz([x, y + 1, z]),
                Face::Xy([x, y, z]),
                Face::Xy([x, y, z + 1]),
            ]
        })
        .counts()
        .values()
        .filter(|count| *count == &1)
        .count()
}

pub fn part_2(input: &str) -> usize {
    let lava: HashSet<_> = input
        .lines()
        .map(|line| -> [isize; 3] {
            line.split(',')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect();
    let search_bounds = [0, 1, 2].map(|i| coordinate_bounds(&lava, i));
    let mut queue = Vec::from([search_bounds
        .clone()
        .map(|coordinate_bounds| coordinate_bounds.start)]);
    let mut seen: HashSet<[isize; 3]> = HashSet::new();
    let mut surface: HashSet<Face> = HashSet::new();
    while let Some([x, y, z]) = queue.pop() {
        let neighbors = [
            ([x - 1, y, z], Face::Yz([x, y, z])),
            ([x + 1, y, z], Face::Yz([x + 1, y, z])),
            ([x, y - 1, z], Face::Xz([x, y, z])),
            ([x, y + 1, z], Face::Xz([x, y + 1, z])),
            ([x, y, z - 1], Face::Xy([x, y, z])),
            ([x, y, z + 1], Face::Xy([x, y, z + 1])),
        ];
        let neighbors = neighbors
            .into_iter()
            .filter_map(|(coordinates, face)| {
                if lava.contains(&coordinates) {
                    surface.insert(face);
                    None
                } else {
                    Some(coordinates)
                }
            })
            .filter(|coordinates| {
                search_bounds
                    .iter()
                    .enumerate()
                    .all(|(i, coordinate_bounds)| coordinate_bounds.contains(&coordinates[i]))
            })
            .filter(|coordinates| !seen.contains(coordinates.as_ref()))
            .collect::<Vec<_>>();
        queue.extend_from_slice(&neighbors);
        seen.extend(neighbors);
    }
    surface.len()
}

fn coordinate_bounds(lava: &HashSet<[isize; 3]>, i: usize) -> Range<isize> {
    let (min_coordinate, max_coordinate) = lava
        .iter()
        .map(|coordinates| coordinates[i])
        .minmax()
        .into_option()
        .unwrap();
    min_coordinate - 1..max_coordinate + 2
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    const INPUT: &str = include_str!("../../../testdata/year_2022/day_18.txt");

    #[test_case(EXAMPLE, 64)]
    #[test_case(INPUT, 4418)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 58)]
    #[test_case(INPUT, 2486)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
