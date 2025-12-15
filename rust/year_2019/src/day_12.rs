use itertools::Itertools as _;
use num::integer::lcm;

pub fn part_1(input: &str, steps: usize) -> usize {
    let [positions, velocities] = parse_moons(input)
        .map(|mut pos| {
            let mut vel = [0; 4];
            for _ in 0..steps {
                step(&mut vel, &mut pos);
            }
            [pos, vel]
        })
        .transpose()
        .map(transpose);

    positions
        .into_iter()
        .zip(velocities)
        .map(|(ps, vs)| {
            let pe: usize = ps.into_iter().map(|p| p.unsigned_abs()).sum();
            let ke: usize = vs.into_iter().map(|v| v.unsigned_abs()).sum();
            pe * ke
        })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    parse_moons(input)
        .into_iter()
        .map(|init: [isize; 4]| {
            let mut vel = [0; 4];
            let mut pos = init;
            let mut steps = 0;
            while {
                step(&mut vel, &mut pos);
                steps += 1;
                pos != init || vel != [0; 4]
            } {}
            steps
        })
        .reduce(lcm)
        .unwrap()
}

/// Unrolled update to velocity and position for 4 moons along a single axis.
#[inline(always)]
fn step(vel: &mut [isize; 4], pos: &mut [isize; 4]) {
    let [p0, p1, p2, p3] = *pos;
    let s01 = p1.cmp(&p0) as isize;
    let s02 = p2.cmp(&p0) as isize;
    let s03 = p3.cmp(&p0) as isize;
    let s12 = p2.cmp(&p1) as isize;
    let s13 = p3.cmp(&p1) as isize;
    let s23 = p3.cmp(&p2) as isize;

    vel[0] += s01 + s02 + s03;
    vel[1] -= s01 - s12 - s13;
    vel[2] -= s02 + s12 - s23;
    vel[3] -= s03 + s13 + s23;

    pos[0] += vel[0];
    pos[1] += vel[1];
    pos[2] += vel[2];
    pos[3] += vel[3];
}

/// Parses the input into three arrays of four positions each, one per axis.
fn parse_moons(input: &str) -> [[isize; 4]; 3] {
    use std::sync::OnceLock;
    static RE: OnceLock<regex::Regex> = OnceLock::new();
    let re = RE.get_or_init(|| regex::Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap());
    re.captures_iter(input)
        .map(|cap| [1, 2, 3].map(|i| cap[i].parse().unwrap()))
        .collect_array()
        .unwrap()
        .transpose()
}

/// Transposes a 2D array of size MxN into one of size NxM.
fn transpose<const M: usize, const N: usize, T>(m: [[T; N]; M]) -> [[T; M]; N] {
    use std::array::from_fn;
    let mut iters = m.map(|r| r.into_iter());
    from_fn(|_| from_fn(|i| iters[i].next().unwrap()))
}

pub trait Transpose<const R: usize, const C: usize> {
    type Item;
    fn transpose(self) -> [[Self::Item; R]; C];

    #[inline(always)]
    fn tranpose(self) -> [[Self::Item; R]; C]
    where
        Self: Sized,
    {
        self.transpose()
    }
}

impl<T, const R: usize, const C: usize> Transpose<R, C> for [[T; C]; R] {
    type Item = T;

    /// Transposes a 2D array of size MxN into one of size NxM.
    #[inline(always)]
    fn transpose(self) -> [[T; R]; C] {
        transpose::<R, C, T>(self)
    }
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

    #[test_case(EXAMPLE_1, 10 => 179)]
    #[test_case(EXAMPLE_2, 100 => 1940)]
    #[test_case(INPUT, 1000 => 7928)]
    fn part_1(input: &str, steps: usize) -> usize {
        super::part_1(input, steps)
    }

    #[test_case(EXAMPLE_1 => 2772)]
    #[test_case(EXAMPLE_2 => 4686774924)]
    #[test_case(INPUT => 518311327635164)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
