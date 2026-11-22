use std::{collections::HashMap, iter};

use hashbrown::{
    DefaultHashBuilder, HashSet,
    hash_set::Entry::{Occupied, Vacant},
};
use itertools::Itertools;
use num::Complex;

pub fn part_1(input: &str) -> usize {
    black_tiles(input).len()
}

pub fn part_2(input: &str) -> usize {
    let counts_0: HashMap<_, _, DefaultHashBuilder> = black_tiles(input)
        .into_iter()
        .map(|tile| (tile, 1))
        .collect();

    (0..100)
        .fold(counts_0, |counts_1, _| {
            let mut counts_2 = counts_1
                .keys()
                .flat_map(iter_neighbours)
                .counts_with_hasher(DefaultHashBuilder::default());

            counts_2.retain(|tile, &mut n| n == 2 || n == 1 && counts_1.contains_key(tile));

            counts_2
        })
        .len()
}

fn black_tiles(input: &str) -> HashSet<Complex<i32>> {
    let mut tiles = HashSet::new();
    for instructions in input.lines() {
        let tile = locate_tile(instructions);
        match tiles.entry(tile) {
            Occupied(entry) => {
                entry.remove();
            }
            Vacant(entry) => {
                entry.insert();
            }
        };
    }
    tiles
}

const EAST: Complex<i32> = Complex::new(1, 0);
const SOUTH_EAST: Complex<i32> = Complex::new(1, -1);
const SOUTH_WEST: Complex<i32> = Complex::new(0, -1);
const WEST: Complex<i32> = Complex::new(-1, 0);
const NORTH_EAST: Complex<i32> = Complex::new(0, 1);
const NORTH_WEST: Complex<i32> = Complex::new(-1, 1);

fn locate_tile(instructions: &str) -> Complex<i32> {
    let mut bytes = instructions.bytes();
    iter::from_fn(|| {
        Some(match bytes.next()? {
            b'e' => EAST,
            b's' => match bytes.next().expect("s must be followed") {
                b'e' => SOUTH_EAST,
                b'w' => SOUTH_WEST,
                b => panic!("unexpected byte after s {}", b as char),
            },
            b'w' => WEST,
            b'n' => match bytes.next().expect("n must be followed") {
                b'e' => NORTH_EAST,
                b'w' => NORTH_WEST,
                b => panic!("unexpected byte after n {}", b as char),
            },
            b => panic!("unexpected byte {}", b as char),
        })
    })
    .sum()
}

fn iter_neighbours(tile: &Complex<i32>) -> impl Iterator<Item = Complex<i32>> {
    [EAST, SOUTH_EAST, SOUTH_WEST, WEST, NORTH_EAST, NORTH_WEST]
        .into_iter()
        .map(move |direction| tile + direction)
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_24.txt");

    const EXAMPLE: &str = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

    #[test_case(EXAMPLE => 10)]
    #[test_case(INPUT => 332)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 2208)]
    #[test_case(INPUT => 3900)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
