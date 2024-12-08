use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    let (antennas, n) = parse_input(input);
    let inc_range = |x1: usize, x2: usize, step: usize| -> [Box<dyn Iterator<Item = usize>>; 2] {
        [
            Box::new((x1.checked_sub(step)).into_iter()),
            Box::new((x2 + step < n).then(|| x2 + step).into_iter()),
        ]
    };
    let dec_range = |x1: usize, x2: usize, step: usize| -> [Box<dyn Iterator<Item = usize>>; 2] {
        [
            Box::new((x1 + step < n).then(|| x1 + step).into_iter()),
            Box::new(x2.checked_sub(step).into_iter()),
        ]
    };
    count_antinodes(antennas, inc_range, dec_range)
}

pub fn part_2(input: &str) -> usize {
    let (antennas, n) = parse_input(input);
    let inc_range = |x1: usize, x2: usize, step: usize| -> [Box<dyn Iterator<Item = usize>>; 2] {
        [
            Box::new((0..=x1).rev().step_by(step)),
            Box::new((x2..n).step_by(step)),
        ]
    };
    let dec_range = |x1: usize, x2: usize, step: usize| -> [Box<dyn Iterator<Item = usize>>; 2] {
        [
            Box::new((x1..n).step_by(step)),
            Box::new((0..=x2).rev().step_by(step)),
        ]
    };
    count_antinodes(antennas, inc_range, dec_range)
}

fn parse_input(input: &str) -> (impl Iterator<Item = Vec<(usize, usize)>>, usize) {
    let mat = input.lines().collect_vec();
    let n = mat.len(); // Assume square matrix
    let group_map = mat
        .into_iter()
        .enumerate()
        .flat_map(|(i, line)| line.bytes().enumerate().map(move |(j, c)| (c, (i, j))))
        .filter(|(c, _)| *c != b'.')
        .into_group_map()
        .into_values();
    (group_map, n)
}

fn count_antinodes(
    antennas: impl Iterator<Item = Vec<(usize, usize)>>,
    inc_range: impl Fn(usize, usize, usize) -> [Box<dyn Iterator<Item = usize>>; 2],
    dec_range: impl Fn(usize, usize, usize) -> [Box<dyn Iterator<Item = usize>>; 2],
) -> usize {
    antennas
        .flat_map(|v| {
            v.into_iter()
                .tuple_combinations()
                .flat_map(|((i1, j1), (i2, j2))| {
                    let [is1, is2] = if i2 > i1 {
                        inc_range(i1, i2, i2 - i1)
                    } else {
                        dec_range(i1, i2, i1 - i2)
                    };
                    let [js1, js2] = if j2 > j1 {
                        inc_range(j1, j2, j2 - j1)
                    } else {
                        dec_range(j1, j2, j1 - j2)
                    };
                    [is1.into_iter().zip(js1), is2.into_iter().zip(js2)]
                        .into_iter()
                        .flatten()
                })
        })
        .unique()
        .count()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../../../testdata/year_2024/day_08.txt");

    const EXAMPLE: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test_case(EXAMPLE, 14)]
    #[test_case(INPUT, 291)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 34)]
    #[test_case(INPUT, 1015)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
