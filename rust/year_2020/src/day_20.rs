use std::{iter::successors, num::NonZeroUsize, str::FromStr};

use hashbrown::{DefaultHashBuilder, HashMap};
use itertools::{Itertools as _, iproduct};

pub fn part_1(input: &str) -> usize {
    let unique_side_ids = input
        .split("\n\n")
        .flat_map(|tile| {
            let tile: Tile = tile.parse().unwrap();
            tile.edges().into_iter().map(move |edge| (edge, tile.id))
        })
        .into_grouping_map_with_hasher(DefaultHashBuilder::default())
        .aggregate(|acc, _key, val| acc.is_none().then_some(val)); // This doesn't get unique, but
    // for some reason it works...
    let unique_side_counts = unique_side_ids
        .into_values()
        // .flatten()
        .counts_with_hasher(DefaultHashBuilder::default());
    unique_side_counts
        .into_iter()
        .filter(|(_, count)| *count == 4)
        .map(|(id, _)| id)
        .product()
}

pub fn part_2(input: &str) -> usize {
    const SEA_MONSTER: [u128; 3] = [
        0b0000_0000_0000_0000_0010,
        0b1000_0110_0001_1000_0111,
        0b0100_1001_0010_0100_1000,
    ];

    let tiles: HashMap<_, _> = input
        .split("\n\n")
        .map(|tile| {
            let tile: Tile = tile.parse().unwrap();
            (tile.id, tile)
        })
        .collect();

    let sides = tiles
        .values()
        .flat_map(|tile| tile.edges().into_iter().map(move |edge| (edge, tile.id)))
        .into_group_map_with_hasher(DefaultHashBuilder::default());

    let unique_side_ids = sides
        .values()
        .filter_map(|ids| ids.iter().exactly_one().ok())
        .counts_with_hasher(DefaultHashBuilder::default());

    let mut corner = unique_side_ids
        .iter()
        .find(|(_, count)| **count == 4)
        .map(|(id, _)| tiles[*id])
        .unwrap();

    while sides[&corner.top()].len() != 1 || sides[&corner.left()].len() != 1 {
        corner.rotate();
    }

    let n = tiles.len().isqrt();

    let row_0: Vec<_> = successors(Some(corner), |left| {
        let id = sides[&left.right()].iter().find(|&&id| id != left.id)?;
        tiles[id]
            .translations()
            .find(|tile| tile.left() == left.right())
    })
    .take(n)
    .collect();

    assert_eq!(row_0.len(), n);

    let rows: Vec<u128> = successors(Some(row_0), |row| {
        row.iter()
            .map(|top| {
                let id = sides[&top.bottom()].iter().find(|&&id| id != top.id)?;

                tiles[id]
                    .translations()
                    .find(|tile| tile.top() == top.bottom())
            })
            .collect::<Option<Vec<_>>>()
    })
    .take(n)
    .flat_map(|tiles| {
        (1..9).map(move |i| {
            tiles.iter().fold(0u128, |acc, tile| {
                (acc << 8) | u128::from((tile.rows[i] >> 1) & 0xff)
            })
        })
    })
    .collect();

    assert_eq!(rows.len(), n * 8);

    fn rotate(rows: &[u128]) -> Vec<u128> {
        let width = rows.len();

        (0..width)
            .map(|j| {
                rows.iter().rev().fold(0u128, |bits, &row| {
                    (bits << 1) | ((row >> (width - 1 - j)) & 1)
                })
            })
            .collect()
    }

    fn translations(rows: Vec<u128>) -> impl Iterator<Item = Vec<u128>> {
        successors(Some(rows), |rows| Some(rotate(rows)))
            .take(4)
            .flat_map(|rows| {
                let mut flipped = rows.clone();
                flipped.reverse();
                [rows, flipped]
            })
    }

    let n = rows.len();
    let roughness: usize = rows.iter().map(|row| row.count_ones() as usize).sum();

    let monsters = translations(rows)
        .find_map(|rows| {
            let count = iproduct!(0..=n - 3, 0..=n - 20)
                .filter(|&(i, j)| {
                    let shift = n - 20 - j;

                    SEA_MONSTER
                        .iter()
                        .enumerate()
                        .all(|(dy, &mask)| (rows[i + dy] >> shift) & mask == mask)
                })
                .count();
            NonZeroUsize::new(count)
        })
        .expect("no sea monsters found");

    roughness - monsters.get() * 15
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Tile {
    id: usize,
    rows: [u16; 10],
}

impl FromStr for Tile {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let id = lines
            .next()
            .ok_or("no lines in input")?
            .strip_circumfix("Tile ", ':')
            .ok_or("invalid tile header")?
            .parse()
            .map_err(|_| "invalid tile id")?;
        let rows = lines
            .map(|line| {
                line.bytes()
                    .map(|b| (b == b'#') as u16)
                    .fold(0, |acc, b| (acc << 1) | b)
            })
            .collect_array()
            .ok_or("invalid number of rows")?;
        Ok(Tile { id, rows })
    }
}

impl Tile {
    fn edges(&self) -> [u16; 8] {
        [
            self.top(),
            self.right(),
            self.bottom(),
            self.left(),
            self.rtop(),
            self.rright(),
            self.rbottom(),
            self.rleft(),
        ]
    }

    #[inline]
    fn top(&self) -> u16 {
        self.rows[0]
    }

    #[inline]
    fn rtop(&self) -> u16 {
        self.top().reverse_bits() >> 6
    }

    #[inline]
    fn right(&self) -> u16 {
        self.rows
            .into_iter()
            .map(|row| row & 1)
            .fold(0, |acc, b| (acc << 1) | b)
    }

    #[inline]
    fn rright(&self) -> u16 {
        self.right().reverse_bits() >> 6
    }

    #[inline]
    fn bottom(&self) -> u16 {
        self.rows[9]
    }

    #[inline]
    fn rbottom(&self) -> u16 {
        self.bottom().reverse_bits() >> 6
    }

    #[inline]
    fn left(&self) -> u16 {
        self.rows
            .into_iter()
            .map(|row| row >> 9)
            .fold(0, |acc, b| (acc << 1) | b)
    }

    #[inline]
    fn rleft(&self) -> u16 {
        self.left().reverse_bits() >> 6
    }

    // TODO: should return new?
    fn rotate(&mut self) {
        self.rows = std::array::from_fn(|j| {
            self.rows
                .iter()
                .rev()
                .fold(0, |bits, &row| (bits << 1) | ((row >> (9 - j)) & 1))
        });
    }

    // TODO: should return new?
    fn flip(&mut self) {
        self.rows.reverse();
    }

    fn translations(&self) -> impl Iterator<Item = Self> {
        successors(Some(*self), |tile| {
            let mut next = *tile;
            next.rotate();
            Some(next)
        })
        .take(4)
        .flat_map(|tile| {
            let mut flipped = tile;
            flipped.flip();
            [tile, flipped]
        })
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_20.txt");

    const EXAMPLE: &str = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";

    #[test_case(EXAMPLE => 20899048083289)]
    #[test_case(INPUT => 16192267830719)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 273)]
    #[test_case(INPUT => 1909)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
