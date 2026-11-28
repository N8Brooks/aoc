use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    polymerization(input, 10)
}

pub fn part_2(input: &str) -> usize {
    polymerization(input, 40)
}

fn polymerization(input: &str, steps: usize) -> usize {
    const N: usize = 26;
    const M: usize = N * N;

    let (template, rules) = input.split_once("\n\n").unwrap();

    fn index(a: u8, b: u8) -> usize {
        (a - b'A') as usize * N + (b - b'A') as usize
    }

    let transitions = rules.lines().fold([None; M], |mut acc, line| {
        let (pair, insert) = line.split_once(" -> ").unwrap();
        let (&[a, b], &[c]) = (pair.as_bytes(), insert.as_bytes()) else {
            panic!("invalid rule: {line}");
        };

        acc[index(a, b)] = Some([index(a, c), index(c, b)]);
        acc
    });

    let mut pair_counts = template
        .as_bytes()
        .array_windows()
        .fold([0; M], |mut acc, &[a, b]| {
            acc[index(a, b)] += 1;
            acc
        });

    for _ in 0..steps {
        pair_counts = pair_counts
            .into_iter()
            .zip(transitions)
            .enumerate()
            .filter(|&(_, (count, _))| count != 0)
            .fold([0; M], |mut acc, (i, (count, transition))| {
                if let Some([a, b]) = transition {
                    acc[a] += count;
                    acc[b] += count;
                } else {
                    acc[i] += count;
                }
                acc
            });
    }

    let mut counts = [0; N];
    counts[usize::from(template.as_bytes()[0] - b'A')] = 1;
    for (i, count) in pair_counts.into_iter().enumerate() {
        counts[i % N] += count;
    }

    counts
        .into_iter()
        .filter(|&count| count > 0)
        .minmax()
        .into_option()
        .map(|(min, max)| max - min)
        .unwrap()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_14.txt");

    const EXAMPLE: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test_case(EXAMPLE => 1588)]
    #[test_case(INPUT => 3831)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 2188189693529)]
    #[test_case(INPUT => 5725739914282)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
