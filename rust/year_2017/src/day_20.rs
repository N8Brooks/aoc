use std::{iter::successors, str::FromStr};

use hashbrown::DefaultHashBuilder;
use itertools::{Itertools as _, izip};

pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<Particle>().unwrap())
        .enumerate()
        .min_set_by_key(|(_, particle)| hypot_2(particle.acceleration))
        .into_iter()
        .flat_map(|(i, particle)| {
            particle
                .path()
                .take_while_inclusive(|particle| !particle.diverging())
                .map(move |particle| (i, hypot_2(particle.position)))
        })
        .min_by_key(|&(_, distance)| distance)
        .unwrap()
        .0
}

pub fn part_2(input: &str) -> usize {
    let mut particles: Vec<Particle> = input.lines().map(|line| line.parse().unwrap()).collect();
    for _ in 0..1_000 {
        for particle in &mut particles {
            particle.step();
        }
        let counts = particles
            .iter()
            .map(|particle| particle.position)
            .counts_with_hasher(DefaultHashBuilder::default());
        particles.retain(|particle| counts[&particle.position] == 1);
    }
    particles.len()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Particle {
    position: [i32; 3],
    velocity: [i32; 3],
    acceleration: [i32; 3],
}

impl FromStr for Particle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse(s: &str, prefix: &str) -> Result<[i32; 3], ()> {
            let [x, y, z] = s
                .strip_circumfix(prefix, ">")
                .ok_or(())?
                .split(',')
                .map(|n| n.parse().map_err(|_| ()))
                .collect_array()
                .ok_or(())?;
            Ok([x?, y?, z?])
        }

        let [p, v, a] = s.split(", ").collect_array().ok_or(())?;
        Ok(Self {
            position: parse(p, "p=<")?,
            velocity: parse(v, "v=<")?,
            acceleration: parse(a, "a=<")?,
        })
    }
}

impl Particle {
    fn path(self) -> impl Iterator<Item = Self> {
        successors(Some(self), |particle| {
            let mut next = *particle;
            next.step();
            Some(next)
        })
    }

    fn step(&mut self) {
        for (p, v, a) in izip!(&mut self.position, &mut self.velocity, &self.acceleration) {
            *v += a;
            *p += *v;
        }
    }

    fn diverging(&self) -> bool {
        izip!(self.position, self.velocity, self.acceleration)
            .all(|(p, v, a)| a == 0 || (p.signum() == v.signum() && v.signum() == a.signum()))
    }
}

fn hypot_2(x: [i32; 3]) -> u32 {
    x.into_iter().map(|x| x.unsigned_abs().pow(2)).sum()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_20.txt");

    const EXAMPLE: &str = "p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>
p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>";

    #[test_case(EXAMPLE => 0)]
    #[test_case(INPUT => 308)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 2)]
    #[test_case(INPUT => 504)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
