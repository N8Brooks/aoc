use hashbrown::{HashMap, HashSet};

pub fn part_1(input: &str) -> usize {
    parse_dists(input).into_values().max().unwrap_or(0)
}

pub fn part_2(input: &str) -> usize {
    parse_dists(input)
        .into_values()
        .filter(|&d| d >= 1000)
        .count()
}

fn parse_dists(input: &str) -> HashMap<(isize, isize), usize> {
    const START: (isize, isize) = (0, 0);
    let positions = HashSet::from([START]);
    let mut stack = Vec::new();
    let mut dists = HashMap::from([(START, 0)]);

    input
        .strip_circumfix('^', '$')
        .unwrap()
        .bytes()
        .fold(positions, |positions, b| match b {
            b'N' | b'E' | b'S' | b'W' => {
                let (di, dj) = match b {
                    b'N' => (-1, 0),
                    b'E' => (0, 1),
                    b'S' => (1, 0),
                    b'W' => (0, -1),
                    _ => unreachable!(),
                };
                positions
                    .iter()
                    .map(|pos @ (i, j)| {
                        let dist = dists[pos] + 1;
                        let pos = (i + di, j + dj);
                        dists.entry(pos).or_insert(dist);
                        pos
                    })
                    .collect()
            }
            b'(' => {
                stack.push((positions.clone(), HashSet::new()));
                positions
            }
            b'|' => {
                let (starts, ends) = stack.last_mut().expect("branch without opener");
                ends.extend(positions);
                starts.clone()
            }
            b')' => {
                let (_starts, mut ends) = stack.pop().expect("branch without closer");
                ends.extend(positions);
                ends
            }
            _ => panic!("unexpected byte: {b}"),
        });

    dists
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_20.txt");

    const EXAMPLE_1: &str = "^WNE$";

    const EXAMPLE_2: &str = "^ENWWW(NEEE|SSE(EE|N))$";

    const EXAMPLE_3: &str = "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$";

    const EXAMPLE_4: &str = "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$";

    const EXAMPLE_5: &str = "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$";

    #[test_case(EXAMPLE_1 => 3)]
    #[test_case(EXAMPLE_2 => 10)]
    #[test_case(EXAMPLE_3 => 18)]
    #[test_case(EXAMPLE_4 => 23)]
    #[test_case(EXAMPLE_5 => 31)]
    #[test_case(INPUT => 3574)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(INPUT => 8444)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
