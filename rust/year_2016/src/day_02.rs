use std::num::NonZeroU8;

pub fn part_1(input: &str) -> String {
    const KEYPAD: &[&[Option<NonZeroU8>]] = &[
        &[
            NonZeroU8::new(b'1'),
            NonZeroU8::new(b'2'),
            NonZeroU8::new(b'3'),
        ],
        &[
            NonZeroU8::new(b'4'),
            NonZeroU8::new(b'5'),
            NonZeroU8::new(b'6'),
        ],
        &[
            NonZeroU8::new(b'7'),
            NonZeroU8::new(b'8'),
            NonZeroU8::new(b'9'),
        ],
    ];
    produce_code(input, KEYPAD)
}

pub fn part_2(input: &str) -> String {
    const KEYPAD: &[&[Option<NonZeroU8>]] = &[
        &[None, None, NonZeroU8::new(b'1'), None, None],
        &[
            None,
            NonZeroU8::new(b'2'),
            NonZeroU8::new(b'3'),
            NonZeroU8::new(b'4'),
            None,
        ],
        &[
            NonZeroU8::new(b'5'),
            NonZeroU8::new(b'6'),
            NonZeroU8::new(b'7'),
            NonZeroU8::new(b'8'),
            NonZeroU8::new(b'9'),
        ],
        &[
            None,
            NonZeroU8::new(b'A'),
            NonZeroU8::new(b'B'),
            NonZeroU8::new(b'C'),
            None,
        ],
        &[None, None, NonZeroU8::new(b'D'), None, None],
    ];
    produce_code(input, KEYPAD)
}

fn produce_code(input: &str, keypad: &[&[Option<NonZeroU8>]]) -> String {
    let mut c = NonZeroU8::new(b'5').unwrap();
    let (mut i, mut j) = keypad
        .iter()
        .enumerate()
        .find_map(|(i, row)| row.iter().position(|&b2| Some(c) == b2).map(|j| (i, j)))
        .unwrap();
    input
        .lines()
        .map(|line| {
            (i, j, c) = line.bytes().fold((i, j, c), |(i, j, c), b| {
                let candidate = match b {
                    b'U' => i.checked_sub(1).map(|i| (i, j)),
                    b'D' => Some((i + 1, j)),
                    b'L' => j.checked_sub(1).map(|j| (i, j)),
                    b'R' => Some((i, j + 1)),
                    _ => panic!("invalid instruction: {line}"),
                };
                candidate
                    .and_then(|(i, j)| {
                        keypad
                            .get(i)
                            .and_then(|row| row.get(j))
                            .flatten_ref()
                            .map(|&c| (i, j, c))
                    })
                    .unwrap_or((i, j, c))
            });
            c.get() as char
        })
        .collect()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_02.txt");

    const EXAMPLE: &str = "ULL
RRDDD
LURDL
UUUUD";

    #[test_case(EXAMPLE => "1985")]
    #[test_case(INPUT => "24862")]
    fn part_1(input: &str) -> String {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => "5DB3")]
    #[test_case(INPUT => "46C91")]
    fn part_2(input: &str) -> String {
        super::part_2(input)
    }
}
