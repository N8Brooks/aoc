pub fn part_1(input: &str) -> usize {
    let mut point = 50;
    parse_input(input)
        .map(|diff| {
            point = (point + diff).rem_euclid(100);
            point
        })
        .filter(|&point| point == 0)
        .count()
}

pub fn part_2(input: &str) -> usize {
    let mut p1 = 50;
    parse_input(input)
        .map(|diff| {
            let p0 = p1 + diff.signum();
            p1 += diff;
            (p0.min(p1), p0.max(p1))
        })
        .filter_map(|(start, end)| {
            let first = start + (100 - start.rem_euclid(100)) % 100;
            (first <= end).then(|| end.abs_diff(first) / 100 + 1)
        })
        .sum()
}

fn parse_input(input: &str) -> impl Iterator<Item = isize> {
    input.lines().map(|line| {
        let (rot, dist) = line.split_at(1);
        let dist: isize = dist.parse().unwrap();
        if rot == "R" { dist } else { -dist }
    })
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_01.txt");

    const EXAMPLE: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test_case(EXAMPLE => 3)]
    #[test_case(INPUT => 1195)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 6)]
    #[test_case(INPUT => 6770)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
