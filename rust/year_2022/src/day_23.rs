use std::fmt::Display;

use ahash::{AHashMap, AHashSet};
use itertools::{FoldWhile, Itertools};

#[derive(Clone, Debug)]
struct Elves(AHashSet<[isize; 2]>);

impl From<&str> for Elves {
    fn from(input: &str) -> Self {
        Elves(
            input
                .lines()
                .enumerate()
                .flat_map(|(i, line)| {
                    let i = i as isize;
                    line.bytes()
                        .enumerate()
                        .filter(|(_, byte)| *byte == b'#')
                        .map(move |(j, _)| [i, j as isize])
                })
                .collect::<AHashSet<_>>(),
        )
    }
}

impl Display for Elves {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const MIN_DISPLAY_I: isize = -2;
        const MAX_DISPLAY_I: isize = 9;
        const MIN_DISPLAY_J: isize = -3;
        const MAX_DISPLAY_J: isize = 10;
        let mut lines = [['.'; (MAX_DISPLAY_J - MIN_DISPLAY_J) as usize + 1];
            (MAX_DISPLAY_I - MIN_DISPLAY_I) as usize + 1];
        for [i, j] in &self.0 {
            let i: usize = (i - MIN_DISPLAY_I).try_into().unwrap();
            let j: usize = (j - MIN_DISPLAY_J).try_into().unwrap();
            lines[i][j] = '#';
        }
        let screen = lines
            .iter()
            .map(|line| line.iter().collect::<String>())
            .join("\n");
        write!(f, "{}", screen)
    }
}

impl Elves {
    fn update(&self, index: usize) -> FoldWhile<Elves> {
        let mut moved_elves: AHashMap<_, Vec<_>> = AHashMap::with_capacity(self.0.len());
        let mut stuck_elves = Vec::new();
        for result in self.proposals(index) {
            match result {
                Ok((from_elf, to_elf)) => moved_elves.entry(to_elf).or_default().push(from_elf),
                Err(stuck_elf) => stuck_elves.push(stuck_elf),
            }
        }
        if moved_elves.is_empty() {
            FoldWhile::Done(self.clone())
        } else {
            FoldWhile::Continue(Elves(
                moved_elves
                    .into_iter()
                    .flat_map(|(to_elf, from_elves)| match from_elves.len() {
                        0 => panic!("expected some elves"),
                        1 => vec![to_elf],
                        _ => from_elves,
                    })
                    .chain(stuck_elves)
                    .collect(),
            ))
        }
    }

    fn proposals(
        &self,
        index: usize,
    ) -> impl Iterator<Item = Result<([isize; 2], [isize; 2]), [isize; 2]>> + '_ {
        self.0.iter().copied().map(move |from_elf| {
            let others = Others::new(&from_elf, self);
            if others.is_other_elf() {
                let [i, j] = from_elf;
                let [d_i, d_j] = others.next_valid_direction(index);
                let to_elf = [i + d_i, j + d_j];
                Ok((from_elf, to_elf))
            } else {
                Err(from_elf)
            }
        })
    }

    fn count_empty(&self) -> usize {
        let height = self.axis_len(0);
        let width = self.axis_len(1);
        let area = width * height;
        let occupied = self.0.len();
        area - occupied
    }

    fn axis_len(&self, axis: usize) -> usize {
        let (min_j, max_j) = self
            .0
            .iter()
            .map(|elf| elf[axis])
            .minmax()
            .into_option()
            .unwrap();
        usize::try_from(max_j - min_j).unwrap() + 1
    }
}

struct Others([[Option<bool>; 3]; 3]);

impl Others {
    fn new(elf_0: &[isize; 2], elves: &Elves) -> Others {
        let [i, j] = elf_0;
        let others = (-1..=1)
            .map(|d_i| {
                let i = i + d_i;
                (-1..=1)
                    .map(|d_j| {
                        let elf_1 = [i, j + d_j];
                        (elf_0 != &elf_1).then(|| elves.0.contains(&elf_1))
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Others(others)
    }

    fn is_other_elf(&self) -> bool {
        self.0.iter().flatten().flatten().any(|is_elf| *is_elf)
    }

    fn next_valid_direction(&self, index: usize) -> [isize; 2] {
        let mut valid_directions = self.valid_directions();
        valid_directions.rotate_left(index % 4);
        valid_directions
            .into_iter()
            .find_map(|valid_direction| valid_direction)
            .unwrap_or([0, 0])
    }

    fn valid_directions(&self) -> [Option<[isize; 2]>; 4] {
        let north = self.0[0]
            .iter()
            .all(|is_elf| !is_elf.unwrap())
            .then_some([-1, 0]);
        let south = self.0[2]
            .iter()
            .all(|is_elf| !is_elf.unwrap())
            .then_some([1, 0]);
        let west = self
            .0
            .iter()
            .all(|are_elves| !are_elves[0].unwrap())
            .then_some([0, -1]);
        let east = self
            .0
            .iter()
            .all(|are_elves| !are_elves[2].unwrap())
            .then_some([0, 1]);
        [north, south, west, east]
    }
}

pub fn part_1(input: &str) -> usize {
    (0..10)
        .fold_while(Elves::from(input), |elves, index| elves.update(index))
        .into_inner()
        .count_empty()
}

pub fn part_2(input: &str) -> usize {
    let mut elves = FoldWhile::Continue(Elves::from(input));
    for index in 0.. {
        if let FoldWhile::Continue(elves_inner) = elves {
            elves = elves_inner.update(index);
        } else {
            return index;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

    const INPUT: &str = include_str!("../../../test_data/year_2022/day_23.txt");

    #[test_case(EXAMPLE, 110)]
    #[test_case(INPUT, 3684)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 20)]
    #[test_case(INPUT, 862)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
