use std::iter::successors;

use itertools::{Itertools as _, multizip};

pub fn part_1(input: &str) -> usize {
    resource_score::<50, 50>(input, 10)
}

pub fn part_2(input: &str) -> usize {
    resource_score::<50, 50>(input, 1_000_000_000)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Acre {
    Open = 0,
    Trees = 1,
    Lumberyard = 2,
}

use Acre::*;

impl From<u8> for Acre {
    fn from(b: u8) -> Self {
        match b {
            b'.' => Open,
            b'|' => Trees,
            b'#' => Lumberyard,
            _ => panic!("invalid byte {b}"),
        }
    }
}

pub fn resource_score<const M: usize, const N: usize>(input: &str, t: usize) -> usize
where
    [(); N + 2]:,
    [(); M + 2]:,
{
    let area_0 = parse_area::<M, N>(input);

    if t == 0 {
        return score(area_0);
    }

    let (mu, lambda) = brent_cycle(area_0);

    let steps = if t < mu { t } else { mu + (t - mu) % lambda };

    let mut areas = successors(Some(area_0), |init| step(init).into());
    score(areas.nth(steps).unwrap())
}

fn parse_area<const M: usize, const N: usize>(input: &str) -> [[Acre; N]; M] {
    input
        .lines()
        .map(|line| line.bytes().map(Acre::from).collect_array().unwrap())
        .collect_array()
        .unwrap()
}

fn brent_cycle<const M: usize, const N: usize>(x0: [[Acre; N]; M]) -> (usize, usize)
where
    [(); N + 2]:,
    [(); M + 2]:,
{
    let mut power = 1;
    let mut lambda = 1;

    let mut tortoise = x0;
    let mut hare = step(&x0);

    while tortoise != hare {
        if power == lambda {
            tortoise = hare;
            power <<= 1;
            lambda = 0;
        }
        hare = step(&hare);
        lambda += 1;
    }

    let mut mu: usize = 0;
    tortoise = x0;
    hare = x0;

    for _ in 0..lambda {
        hare = step(&hare);
    }

    while tortoise != hare {
        tortoise = step(&tortoise);
        hare = step(&hare);
        mu += 1;
    }

    (mu, lambda)
}

fn step<const M: usize, const N: usize>(init: &[[Acre; N]; M]) -> [[Acre; N]; M]
where
    [(); N + 2]:,
    [(); M + 2]:,
{
    let area = pad(init);
    area.array_windows()
        .map(|[a, b, c]| {
            multizip((a.array_windows(), b.array_windows(), c.array_windows()))
                .map(|(&[a, b, c], &[d, e, f], &[g, h, i])| {
                    let [_, trees, lumberyards] =
                        [a, b, c, d, f, g, h, i]
                            .into_iter()
                            .fold([0; 3], |mut counts, acre| {
                                counts[acre as usize] += 1;
                                counts
                            });
                    match (e, trees, lumberyards) {
                        (Open, 3.., _) => Trees,
                        (Trees, _, 3..) => Lumberyard,
                        (Lumberyard, 0, _) | (Lumberyard, _, 0) => Open,
                        _ => e,
                    }
                })
                .collect_array()
                .unwrap()
        })
        .collect_array()
        .unwrap()
}

fn pad<const M: usize, const N: usize>(init: &[[Acre; N]; M]) -> [[Acre; N + 2]; M + 2] {
    let mut area = [[Open; N + 2]; M + 2];
    area.iter_mut().skip(1).zip(init).for_each(|(area, init)| {
        area[1..=N].copy_from_slice(init);
    });
    area
}

fn score<const M: usize, const N: usize>(area_n: [[Acre; N]; M]) -> usize {
    let [_, trees, lumberyards] = area_n
        .into_iter()
        .flatten()
        .fold([0; 3], |mut counts, acre| {
            counts[acre as usize] += 1;
            counts
        });
    trees * lumberyards
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_18.txt");

    const EXAMPLE: &str = "\
.#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.";

    #[test_case(EXAMPLE => 1147)]
    fn resource_score(input: &str) -> usize {
        super::resource_score::<10, 10>(input, 10)
    }

    #[test_case(INPUT => 637550)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(INPUT => 201465)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
