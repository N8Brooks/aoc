use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    parse_triangles(input).filter(is_triangle).count()
}

pub fn part_2(input: &str) -> usize {
    parse_triangles(input)
        .array_chunks()
        .flat_map(transpose)
        .filter(is_triangle)
        .count()
}

fn parse_triangles(input: &str) -> impl Iterator<Item = [u16; 3]> {
    input.lines().map(|line| {
        line.split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect_array()
            .unwrap()
    })
}

#[inline]
const fn is_triangle(&[a, b, c]: &[u16; 3]) -> bool {
    a + b > c && a + c > b && b + c > a
}

/// Transposes a 2D array of size MxN into one of size NxM.
fn transpose<const M: usize, const N: usize, T>(m: [[T; N]; M]) -> [[T; M]; N] {
    use std::array::from_fn;
    let mut iters = m.map(|r| r.into_iter());
    from_fn(|_| from_fn(|i| iters[i].next().unwrap()))
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_03.txt");

    #[test_case("5 10 25" => 0)]
    #[test_case(INPUT => 869)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(INPUT => 1544)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
