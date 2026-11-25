pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .flat_map(|line| line.rsplit_once(" | ").unwrap().1.split(' '))
        .filter(|digit| matches!(digit.len(), 2 | 3 | 4 | 7))
        .count()
}

pub fn part_2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let (patterns, output) = line.split_once(" | ").unwrap();
            let one = mask(patterns.split(' ').find(|p| p.len() == 2).unwrap());
            let four = mask(patterns.split(' ').find(|p| p.len() == 4).unwrap());
            output
                .split_ascii_whitespace()
                .map(|digit| {
                    let bits = mask(digit);
                    match digit.len() {
                        2 => 1,
                        3 => 7,
                        4 => 4,
                        5 if bits & one == one => 3,
                        5 if (bits & four).count_ones() == 2 => 2,
                        5 => 5,
                        6 if bits & four == four => 9,
                        6 if bits & one == one => 0,
                        6 => 6,
                        7 => 8,
                        _ => unreachable!(),
                    }
                })
                .fold(0, |acc, d| acc * 10 + d)
        })
        .sum()
}

fn mask(pattern: &str) -> u8 {
    pattern.bytes().fold(0, |acc, b| acc | (1 << (b - b'a')))
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_08.txt");

    const EXAMPLE_1: &str =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

    const EXAMPLE_2: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test_case(EXAMPLE_2 => 26)]
    #[test_case(INPUT => 237)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE_1 => 5353)]
    #[test_case(EXAMPLE_2 => 61229)]
    #[test_case(INPUT => 1009098)]
    fn part_2(input: &str) -> u64 {
        super::part_2(input)
    }
}
