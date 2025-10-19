use itertools::{Itertools as _, multizip};
use num::integer::lcm;

pub fn part_1(input: &str, steps: usize) -> usize {
    let mut positions = parse_moons(input);
    let mut velocities = [[0; 3]; 4];

    for _ in 0..steps {
        for i in 0..4 {
            for j in (i + 1)..4 {
                let (v1s, v2s) = velocities.split_at_mut(j);
                let (v1s, v2s) = (&mut v1s[i], &mut v2s[0]);
                let (p1s, p2s) = (positions[i], positions[j]);
                for (v1, p1, v2, p2) in multizip((v1s, p1s, v2s, p2s)) {
                    *v1 += (p2 - p1).signum();
                    *v2 += (p1 - p2).signum();
                }
            }
        }

        for (ps, vs) in positions.iter_mut().zip(velocities) {
            for (p, v) in ps.iter_mut().zip(vs) {
                *p += v;
            }
        }
    }

    positions
        .into_iter()
        .zip(velocities)
        .map(|(ps, vs)| {
            let potential_energy: usize = ps.into_iter().map(|p| p.unsigned_abs()).sum();
            let kinetic_energy: usize = vs.into_iter().map(|v| v.unsigned_abs()).sum();
            potential_energy * kinetic_energy
        })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    let initial_positions = parse_moons(input);
    let mut positions = initial_positions;
    let mut velocities = [[0; 3]; 4];
    let mut periods = [0; 3];

    for step in 1.. {
        for i in 0..4 {
            for j in (i + 1)..4 {
                let (v1s, v2s) = velocities.split_at_mut(j);
                let (v1s, v2s) = (&mut v1s[i], &mut v2s[0]);
                let (p1s, p2s) = (positions[i], positions[j]);
                for (v1, p1, v2, p2) in multizip((v1s, p1s, v2s, p2s)) {
                    *v1 += (p2 - p1).signum();
                    *v2 += (p1 - p2).signum();
                }
            }
        }

        for (ps, vs) in positions.iter_mut().zip(velocities) {
            for (p, v) in ps.iter_mut().zip(vs) {
                *p += v;
            }
        }

        for dim in 0..3 {
            if periods[dim] == 0
                && positions.iter().zip(&velocities).all(|(pos, vel)| {
                    pos[dim]
                        == initial_positions[positions.iter().position(|p| p == pos).unwrap()][dim]
                        && vel[dim] == 0
                })
            {
                periods[dim] = step;
            }
        }

        if !periods.contains(&0) {
            break;
        }
    }

    periods.into_iter().reduce(lcm).unwrap()
}

fn parse_moons(input: &str) -> [[isize; 3]; 4] {
    use std::sync::OnceLock;
    static RE: OnceLock<regex::Regex> = OnceLock::new();
    let re = RE.get_or_init(|| regex::Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap());
    re.captures_iter(input)
        .map(|cap| {
            [
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
                cap[3].parse().unwrap(),
            ]
        })
        .collect_array()
        .unwrap()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE_1: &str = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";

    const EXAMPLE_2: &str = "<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>";

    const INPUT: &str = include_str!("../test_data/day_12.txt");

    #[test_case(EXAMPLE_1, 179, 10)]
    #[test_case(EXAMPLE_2, 1940, 100)]
    #[test_case(INPUT, 7928, 1000)]
    fn part_1(input: &str, expected: usize, steps: usize) {
        assert_eq!(super::part_1(input, steps), expected);
    }

    #[test_case(EXAMPLE_1, 2772)]
    #[test_case(EXAMPLE_2, 4686774924)]
    #[test_case(INPUT, 518311327635164)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
