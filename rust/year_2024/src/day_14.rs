use std::cmp::Ordering;

use itertools::Itertools as _;

pub fn part_1(input: &str, m: i32, n: i32) -> i32 {
    let mid_m = m / 2;
    let mid_n = n / 2;
    parse_input(input)
        .filter_map(|(i, j, di, dj)| {
            let i = (i + di * 100).rem_euclid(m);
            let j = (j + dj * 100).rem_euclid(n);
            match (i.cmp(&mid_m), j.cmp(&mid_n)) {
                (Ordering::Less, Ordering::Less) => Some(0),
                (Ordering::Less, Ordering::Greater) => Some(1),
                (Ordering::Greater, Ordering::Less) => Some(2),
                (Ordering::Greater, Ordering::Greater) => Some(3),
                _ => None,
            }
        })
        .fold([0; 4], |mut acc, q| {
            acc[q] += 1;
            acc
        })
        .into_iter()
        .product()
}

pub fn part_2(input: &str, m: i32, n: i32) -> i32 {
    let robots = parse_input(input);
    let (is, js, dis, djs): (Vec<_>, Vec<_>, Vec<_>, Vec<_>) = robots.multiunzip();
    let [ti, tj] = [(is, dis, m), (js, djs, n)].map(|(mut xs, dxs, n)| {
        let mut ti = 0;
        let threshold = n as f32 / 4.;
        while maths::std(&xs) > threshold {
            xs.iter_mut()
                .zip(&dxs)
                .for_each(|(i, di)| *i = (*i + di).rem_euclid(n));
            ti += 1;
        }
        ti
    });
    maths::chinese_remainder_theorem(&[ti, tj], &[m, n]).unwrap()
}

fn parse_input(input: &str) -> impl Iterator<Item = (i32, i32, i32, i32)> + '_ {
    input.lines().map(|line| {
        let (p, v) = line.split_once(" ").unwrap();
        let p = p.strip_prefix("p=").unwrap();
        let v = v.strip_prefix("v=").unwrap();
        let (pj, pi) = p.split_once(",").unwrap();
        let (vj, vi) = v.split_once(",").unwrap();
        (
            pi.parse().unwrap(),
            pj.parse().unwrap(),
            vi.parse().unwrap(),
            vj.parse().unwrap(),
        )
    })
}

mod maths {
    pub(super) fn std(values: &[i32]) -> f32 {
        let n = values.len() as f32;
        let mean = values.iter().sum::<i32>() as f32 / n;
        let var = values
            .iter()
            .map(|&x| (x as f32 - mean).powi(2))
            .sum::<f32>()
            / n;
        var.sqrt()
    }

    /// Source: https://github.com/TheAlgorithms/
    pub(super) fn chinese_remainder_theorem(residues: &[i32], modulli: &[i32]) -> Option<i32> {
        let prod = modulli.iter().product::<i32>();
        let mut sum = 0;
        for (&residue, &modulus) in residues.iter().zip(modulli) {
            let p = prod / modulus;
            sum += residue * mod_inv(p, modulus)? * p
        }
        Some(sum % prod)
    }

    fn mod_inv(x: i32, n: i32) -> Option<i32> {
        let (g, x, _) = extended_euclidean_algorithm(x, n);
        if g == 1 {
            Some((x % n + n) % n)
        } else {
            None
        }
    }

    fn extended_euclidean_algorithm(a: i32, b: i32) -> (i32, i32, i32) {
        let (mut old_r, mut rem) = (a, b);
        let (mut old_s, mut coeff_s) = (1, 0);
        let (mut old_t, mut coeff_t) = (0, 1);
        while rem != 0 {
            let quotient = old_r / rem;
            update_step(&mut rem, &mut old_r, quotient);
            update_step(&mut coeff_s, &mut old_s, quotient);
            update_step(&mut coeff_t, &mut old_t, quotient);
        }
        (old_r, old_s, old_t)
    }

    fn update_step(a: &mut i32, old_a: &mut i32, quotient: i32) {
        let temp = *a;
        *a = *old_a - quotient * temp;
        *old_a = temp;
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_14.txt");

    const EXAMPLE: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test_case(EXAMPLE, 7, 11, 12)]
    #[test_case(INPUT, 103, 101, 230900224)]
    fn part_1(input: &str, m: i32, n: i32, expected: i32) {
        assert_eq!(super::part_1(input, m, n), expected);
    }

    #[test_case(INPUT, 103, 101, 6532)]
    fn part_2(input: &str, m: i32, n: i32, expected: i32) {
        assert_eq!(super::part_2(input, m, n), expected);
    }
}
