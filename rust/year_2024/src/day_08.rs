use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    let (antennas, (m, n)) = parse_input(input);
    antennas
        .flat_map(|v| {
            v.into_iter()
                .tuple_combinations()
                .flat_map(|((i1, j1), (i2, j2))| {
                    let di = i2 - i1;
                    let dj = j2 - j1;
                    let [is1, is2]: [Box<dyn Iterator<Item = isize>>; 2] = if di >= 0 {
                        let di = di as usize;
                        [
                            Box::new((0..=i1).rev().step_by(di).skip(1).take(1)),
                            Box::new((i2..m).step_by(di).skip(1).take(1)),
                        ]
                    } else {
                        let di = -di as usize;
                        [
                            Box::new((i1..m).step_by(di).skip(1).take(1)),
                            Box::new((0..=i2).rev().step_by(di).skip(1).take(1)),
                        ]
                    };
                    let [js1, js2]: [Box<dyn Iterator<Item = isize>>; 2] = if dj >= 0 {
                        let dj = dj as usize;
                        [
                            Box::new((0..=j1).rev().step_by(dj).skip(1).take(1)),
                            Box::new((j2..n).step_by(dj).skip(1).take(1)),
                        ]
                    } else {
                        let dj = -dj as usize;
                        [
                            Box::new((j1..n).step_by(dj).skip(1).take(1)),
                            Box::new((0..=j2).rev().step_by(dj).skip(1).take(1)),
                        ]
                    };
                    [is1.into_iter().zip(js1), is2.into_iter().zip(js2)]
                        .into_iter()
                        .flatten()
                })
        })
        .unique()
        .count()
}

pub fn part_2(input: &str) -> usize {
    let (antennas, (m, n)) = parse_input(input);
    antennas
        .flat_map(|v| {
            v.into_iter()
                .tuple_combinations()
                .flat_map(|((i1, j1), (i2, j2))| {
                    let di = i2 - i1;
                    let dj = j2 - j1;
                    let [is1, is2]: [Box<dyn Iterator<Item = isize>>; 2] = if di >= 0 {
                        let di = di as usize;
                        [
                            Box::new((0..=i1).rev().step_by(di)),
                            Box::new((i2..m).step_by(di)),
                        ]
                    } else {
                        let di = -di as usize;
                        [
                            Box::new((i1..m).step_by(di)),
                            Box::new((0..=i2).rev().step_by(di)),
                        ]
                    };
                    let [js1, js2]: [Box<dyn Iterator<Item = isize>>; 2] = if dj >= 0 {
                        let dj = dj as usize;
                        [
                            Box::new((0..=j1).rev().step_by(dj)),
                            Box::new((j2..n).step_by(dj)),
                        ]
                    } else {
                        let dj = -dj as usize;
                        [
                            Box::new((j1..n).step_by(dj)),
                            Box::new((0..=j2).rev().step_by(dj)),
                        ]
                    };
                    [is1.into_iter().zip(js1), is2.into_iter().zip(js2)]
                        .into_iter()
                        .flatten()
                })
        })
        .unique()
        .count()
}

fn parse_input(input: &str) -> (impl Iterator<Item = Vec<(isize, isize)>>, (isize, isize)) {
    let mat = input.lines().collect_vec();
    let m = mat.len() as isize;
    let n = mat[0].len() as isize;
    let group_map = mat
        .into_iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.bytes()
                .enumerate()
                .map(move |(j, c)| (c, (i as isize, j as isize)))
        })
        .filter(|(c, _)| *c != b'.')
        .into_group_map()
        .into_values();
    (group_map, (m, n))
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
