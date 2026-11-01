use std::iter::repeat_n;

use hashbrown::HashMap;

pub fn part_1(input: &str) -> usize {
    let n: usize = input.parse().unwrap();
    if n == 1 {
        return 0;
    }
    let ring = (n - 1).isqrt().div_ceil(2);
    let side = 2 * ring;
    let prev_max = (side - 1).pow(2);
    ring + ((n - prev_max) % side).abs_diff(ring)
}

pub fn part_2(input: &str) -> usize {
    let n: usize = input.parse().unwrap();
    let mut grid: HashMap<(isize, isize), usize> = HashMap::from([((0, 0), 1)]);
    for (i, j) in spiral() {
        let value = (-1..=1)
            .flat_map(|di| (-1..=1).map(move |dj| (i + di, j + dj)))
            .filter_map(|pos| grid.get(&pos))
            .sum();
        if value > n {
            return value;
        }
        grid.insert((i, j), value);
    }
    unreachable!()
}

fn spiral() -> impl Iterator<Item = (isize, isize)> {
    [(0, 1), (-1, 0), (0, -1), (1, 0)]
        .into_iter()
        .cycle()
        .zip((1..).flat_map(|len| repeat_n(len, 2)))
        .flat_map(|(delta, len)| repeat_n(delta, len))
        .scan((0, 0), |pos, (di, dj)| {
            pos.0 += di;
            pos.1 += dj;
            Some(*pos)
        })
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_03.txt");

    #[test_case("1" => 0)]
    #[test_case("12" => 3)]
    #[test_case("23" => 2)]
    #[test_case("1024" => 31)]
    #[test_case(INPUT => 480)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(INPUT => 349975)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
