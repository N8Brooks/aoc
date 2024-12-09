use phf::phf_map;
use std::str::Lines;

struct RegisterXParser<'a> {
    input_lines: Lines<'a>,
    register_x: i64,
}

impl Iterator for RegisterXParser<'_> {
    type Item = Vec<i64>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(line) = self.input_lines.next() {
            let register_xs = match line.split_once(' ') {
                // addx
                Some((_, v)) => {
                    let register_xs = vec![self.register_x; 2];
                    self.register_x += v.parse::<i64>().unwrap();
                    register_xs
                }
                // noop
                None => vec![self.register_x],
            };
            Some(register_xs)
        } else {
            None
        }
    }
}

impl<'a> RegisterXParser<'a> {
    fn new(input: &'a str) -> RegisterXParser<'a> {
        RegisterXParser {
            input_lines: input.lines(),
            register_x: 1,
        }
    }
}

pub fn part_1(input: &str) -> i64 {
    RegisterXParser::new(input)
        .flatten()
        .enumerate()
        .filter_map(|(i, register_x)| {
            let cycle = i as i64 + 1;
            if (cycle - 20) % 40 == 0 {
                Some(cycle * register_x)
            } else {
                None
            }
        })
        .sum()
}

/// Maps letter strings to their char representation.
static LETTERS: phf::Map<&'static str, char> = phf_map! {
    // Some random characters for example test case
    "##..#\n###..\n####.\n#####\n#####\n#####" => '◣',
    "#..##\n.###.\n...##\n.....\n#....\n##..." => '◂',
    "..##.\n..###\n##...\n#####\n..###\n....#" => '◄',
    ".##..\n...##\n.####\n.....\n###..\n#####" => '◈',
    "##..#\n#...#\n....#\n#####\n....#\n#...." => '▿',
    "#..##\n##...\n###..\n.....\n#####\n...##" => '▽',
    "..##.\n###..\n..###\n#####\n.....\n#####" => '◅',
    ".##..\n.###.\n#....\n.....\n.####\n....." => '◇',
    // Actual letters for input test case
    "###..\n#..#.\n#..#.\n###..\n#.#..\n#..#." => 'R',
    "#..#.\n#.#..\n##...\n#.#..\n#.#..\n#..#." => 'K',
    "###..\n#..#.\n#..#.\n###..\n#....\n#...." => 'P',
    "..##.\n...#.\n...#.\n...#.\n#..#.\n.##.." => 'J',
    "###..\n#..#.\n###..\n#..#.\n#..#.\n###.." => 'B',
    "#....\n#....\n#....\n#....\n#....\n####." => 'L',
    ".##..\n#..#.\n#..#.\n####.\n#..#.\n#..#." => 'A',
};

pub fn part_2(input: &str) -> String {
    // 8 capital letters where each capital letter has 6 rows of length 5
    let mut letters = vec![vec![String::new(); 6]; 8];
    for (i, register_x) in RegisterXParser::new(input).flatten().enumerate() {
        let position = i % 40;
        let pixel = if register_x.abs_diff(position as i64) > 1 {
            '.'
        } else {
            '#'
        };
        letters[position / 5][i / 40].push(pixel);
    }
    letters
        .iter()
        .map(|letter| LETTERS[letter.join("\n").as_str()])
        .collect()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    const INPUT: &str = include_str!("../../../test_data/year_2022/day_10.txt");

    #[test_case(EXAMPLE, 13140)]
    #[test_case(INPUT, 15120)]
    fn part_1(input: &str, expected: i64) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, "◣◂◄◈▿▽◅◇")]
    #[test_case(INPUT, "RKPJBPLA")]
    fn part_2(input: &str, expected: &str) {
        assert_eq!(super::part_2(input), expected);
    }
}
