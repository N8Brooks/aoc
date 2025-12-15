use hashbrown::HashSet;
use itertools::Itertools as _;
use num::Integer as _;

pub fn part_1(input: &str) -> usize {
    let asteroids = parse_asteroids(input);
    asteroids
        .iter()
        .map(|(x1, y1)| {
            asteroids
                .iter()
                .filter(|&(x2, y2)| x1 != x2 || y1 != y2)
                .map(|(x2, y2)| angle(x1 - x2, y1 - y2))
                .collect::<HashSet<_>>()
                .len()
        })
        .max()
        .unwrap()
}

pub fn part_2(input: &str) -> usize {
    let asteroids = parse_asteroids(input);
    let &(x0, y0) = asteroids
        .iter()
        .max_by_key(|(x1, y1)| {
            asteroids
                .iter()
                .filter(|(x2, y2)| x1 != x2 || y1 != y2)
                .map(|(x2, y2)| angle(x1 - x2, y1 - y2))
                .collect::<HashSet<_>>()
                .len()
        })
        .unwrap();

    // Find the first asteroid vaporized in each direction
    let mut firsts = asteroids
        .into_iter()
        .filter(|&(x1, y1)| x0 != x1 || y0 != y1)
        .into_grouping_map_by(|(x1, y1)| angle(x0 - x1, y0 - y1))
        .min_by_key(|_, &(x1, y1)| x0.abs_diff(x1) + y0.abs_diff(y1))
        .into_values()
        .collect_vec();

    // Find the 200th asteroid vaporized (panics if fewer than 200)
    let (x1, y1) = *firsts
        .select_nth_unstable_by(199, |(x1, y1), (x2, y2)| {
            let (dx1, dy1) = (x1 - x0, y1 - y0);
            let (dx2, dy2) = (x2 - x0, y2 - y0);
            let (h1, h2) = (dx1 < 0, dx2 < 0);
            h1.cmp(&h2)
                .then_with(|| (dx1 * dy2 - dy1 * dx2).cmp(&0).reverse())
                .then_with(|| dy1.cmp(&dy2))
        })
        .1;

    100 * usize::try_from(x1).unwrap() + usize::try_from(y1).unwrap()
}

fn parse_asteroids(input: &str) -> Vec<(isize, isize)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes()
                .enumerate()
                .filter(|(_, b)| *b == b'#')
                .map(move |(x, _)| (x.try_into().unwrap(), y.try_into().unwrap()))
        })
        .collect_vec()
}

#[inline]
fn angle(dx: isize, dy: isize) -> (isize, isize) {
    let gcd = dx.abs().gcd(&dy.abs());
    (dx / gcd, dy / gcd)
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE_1: &str = ".#..#
.....
#####
....#
...##";

    const EXAMPLE_2: &str = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";

    const EXAMPLE_3: &str = "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";

    const EXAMPLE_4: &str = ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..";

    const EXAMPLE_5: &str = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

    const INPUT: &str = include_str!("../test_data/day_10.txt");

    #[test_case(EXAMPLE_1 => 8)]
    #[test_case(EXAMPLE_2 => 33)]
    #[test_case(EXAMPLE_3 => 35)]
    #[test_case(EXAMPLE_4 => 41)]
    #[test_case(EXAMPLE_5 => 210)]
    #[test_case(INPUT => 267)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(INPUT => 1309)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
