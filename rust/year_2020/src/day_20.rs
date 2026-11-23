use std::iter::successors;

use itertools::Itertools as _;

type Tile = [u128; 10];
type Sides = [Vec<usize>; 1 << 10];

pub fn part_1(input: &str) -> usize {
    let (tiles, sides) = parse(input);
    tiles
        .iter()
        .filter(|(_, tile)| corner(tile, &sides))
        .map(|(id, _)| id)
        .product()
}

pub fn part_2(input: &str) -> usize {
    let (tiles, sides) = parse(input);
    let n = tiles.len().isqrt();
    let mut board: Vec<(usize, Tile)> = Vec::with_capacity(tiles.len());
    for pos in 0..tiles.len() {
        let left = (!pos.is_multiple_of(n)).then(|| {
            let (id, tile) = &board[pos - 1];
            (*id, column(tile, 0))
        });
        let above = pos.checked_sub(n).map(|i| (board[i].0, board[i].1[9]));
        let next = match left.or(above) {
            Some((prev, edge)) => *sides[edge_key(edge)].iter().find(|&&i| i != prev).unwrap(),
            None => tiles
                .iter()
                .position(|(_, tile)| corner(tile, &sides))
                .unwrap(),
        };
        let fits = |edge, neighbor: Option<(usize, u128)>| {
            neighbor.map_or_else(
                || sides[edge_key(edge)].len() == 1,
                |(_, other)| edge == other,
            )
        };
        let tile = orientations(tiles[next].1)
            .find(|t| fits(t[0], above) && fits(column(t, 9), left))
            .unwrap();
        board.push((next, tile));
    }
    let rows = board
        .chunks_exact(n)
        .flat_map(|tiles| {
            (1..9).map(move |y| {
                tiles
                    .iter()
                    .fold(0, |row, (_, t)| (row << 8) | ((t[y] >> 1) & 0xff))
            })
        })
        .collect_vec();
    let roughness: u32 = rows.iter().map(|r| r.count_ones()).sum();
    let monsters = orientations(rows)
        .map(|rows| -> u32 {
            // Each surviving bit marks the right edge of a monster in this band of rows.
            rows.array_windows()
                .map(|&[a, b, c]| {
                    let middle = [0, 1, 2, 7, 8, 13, 14, 19]
                        .into_iter()
                        .fold(a >> 1, |hits, dx| hits & (b >> dx));
                    [3, 6, 9, 12, 15, 18]
                        .into_iter()
                        .fold(middle, |hits, dx| hits & (c >> dx))
                        .count_ones()
                })
                .sum()
        })
        .find(|&count| count > 0)
        .expect("no sea monsters found");
    (roughness - 15 * monsters) as usize
}

fn parse(input: &str) -> (Vec<(usize, Tile)>, Sides) {
    let tiles = input
        .split("\n\n")
        .map(|block| {
            let (header, rows) = block.split_once('\n').unwrap();
            let id = header
                .strip_circumfix("Tile ", ':')
                .unwrap()
                .parse()
                .unwrap();
            let rows = rows
                .lines()
                .map(|row| {
                    row.bytes()
                        .fold(0, |bits, b| (bits << 1) | u128::from(b == b'#'))
                })
                .collect_array()
                .unwrap();
            (id, rows)
        })
        .collect_vec();
    let mut sides = std::array::from_fn(|_| Vec::new());
    for (i, (_, tile)) in tiles.iter().enumerate() {
        for edge in edges(tile) {
            sides[edge_key(edge)].push(i);
        }
    }
    (tiles, sides)
}

fn corner(tile: &Tile, sides: &Sides) -> bool {
    edges(tile)
        .into_iter()
        .filter(|&edge| sides[edge_key(edge)].len() == 1)
        .count()
        == 2
}

fn column(rows: &[u128], bit: usize) -> u128 {
    rows.iter()
        .fold(0, |col, row| (col << 1) | ((row >> bit) & 1))
}

fn edges(tile: &Tile) -> [u128; 4] {
    [tile[0], column(tile, 0), tile[9], column(tile, 9)]
}

fn edge_key(edge: u128) -> usize {
    edge.min(edge.reverse_bits() >> (u128::BITS - 10)) as usize
}

fn orientations<T: AsRef<[u128]> + AsMut<[u128]> + Clone>(image: T) -> impl Iterator<Item = T> {
    successors(Some(image), |image| {
        let rows = image.as_ref();
        let mut rotated = image.clone();
        for (bit, row) in rotated.as_mut().iter_mut().enumerate() {
            *row = column(rows, bit);
        }
        Some(rotated)
    })
    .take(4)
    .flat_map(|image| {
        let mut flipped = image.clone();
        flipped.as_mut().reverse();
        [image, flipped]
    })
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
