use itertools::Itertools as _;
use num::Integer as _;

pub fn part_1(input: &str) -> u32 {
    max_distance_1(input, 2503)
}

fn max_distance_1(input: &str, n: u32) -> u32 {
    parse_input(input)
        .map(|(speed, fly, rest)| {
            let cycle = fly + rest;
            let (q, r) = n.div_rem(&cycle);
            let t = q * fly + r.min(fly);
            t * speed
        })
        .max()
        .unwrap()
}

pub fn part_2(input: &str) -> u32 {
    max_distance_2(input, 2503)
}

fn max_distance_2(input: &str, n: u32) -> u32 {
    let reindeers: Vec<_> = parse_input(input).collect();
    (1..=n)
        .fold(vec![0; reindeers.len()], |mut points, t| {
            points
                .iter_mut()
                .zip(&reindeers)
                .max_set_by_key(|&(_, &(speed, fly, res))| {
                    let cycle = fly + res;
                    let (q, r) = t.div_rem(&cycle);
                    let t = q * fly + r.min(fly);
                    t * speed
                })
                .into_iter()
                .for_each(|(points, _)| *points += 1);
            points
        })
        .into_iter()
        .max()
        .unwrap()
}

fn parse_input(input: &str) -> impl Iterator<Item = (u32, u32, u32)> {
    input.lines().map(|line| {
        let (_name, rest) = line.split_once(" can fly ").unwrap();
        let (speed, rest) = rest.split_once(" km/s for ").unwrap();
        let (fly, rest) = rest
            .split_once(" seconds, but then must rest for ")
            .unwrap();
        let rest = rest.strip_suffix(" seconds.").unwrap();
        (
            speed.parse().unwrap(),
            fly.parse().unwrap(),
            rest.parse().unwrap(),
        )
    })
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_14.txt");

    const EXAMPLE: &str =
        "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";

    #[test_case(EXAMPLE => 1120)]
    fn max_distance_1(input: &str) -> u32 {
        super::max_distance_1(input, 1000)
    }

    #[test_case(INPUT => 2696)]
    fn part_1(input: &str) -> u32 {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 689)]
    fn max_distance_2(input: &str) -> u32 {
        super::max_distance_2(input, 1000)
    }

    #[test_case(INPUT => 1084)]
    fn part_2(input: &str) -> u32 {
        super::part_2(input)
    }
}
