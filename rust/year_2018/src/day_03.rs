use itertools::Itertools;

pub fn part_1(input: &str) -> usize {
    let claims = parse_claims(input);
    parse_counts(claims)
        .into_iter()
        .flatten()
        .filter(|&count| count > 1)
        .count()
}

pub fn part_2(input: &str) -> usize {
    let claims: Vec<_> = parse_claims(input).collect();
    let counts = parse_counts(claims.iter().copied());
    claims
        .into_iter()
        .enumerate()
        .filter(|&(_, (j, i, m, n))| {
            counts
                .iter()
                .skip(i)
                .take(n)
                .flat_map(|row| row.iter().skip(j).take(m))
                .all(|&count| count == 1)
        })
        .exactly_one()
        .unwrap()
        .0
        + 1
}

fn parse_claims(input: &str) -> impl Iterator<Item = (usize, usize, usize, usize)> {
    input.lines().map(|line| {
        let (_, line) = line.split_once(" @ ").unwrap();
        let (pos, size) = line.split_once(": ").unwrap();
        let (j, i) = pos.split_once(',').unwrap();
        let (m, n) = size.split_once('x').unwrap();
        (
            j.parse().unwrap(),
            i.parse().unwrap(),
            m.parse().unwrap(),
            n.parse().unwrap(),
        )
    })
}

fn parse_counts(claims: impl IntoIterator<Item = (usize, usize, usize, usize)>) -> Vec<Vec<i32>> {
    let mut counts = vec![vec![0; 1000]; 1000];
    for (j, i, m, n) in claims {
        for row in counts.iter_mut().skip(i).take(n) {
            for count in row.iter_mut().skip(j).take(m) {
                *count += 1;
            }
        }
    }
    counts
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_03.txt");

    const EXAMPLE: &str = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";

    #[test_case(EXAMPLE => 4)]
    #[test_case(INPUT => 121163)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 3)]
    #[test_case(INPUT => 943)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
