use std::iter;

pub fn part_1(input: &str) -> usize {
    let mut bytes = input.bytes();
    let mut depth = 0;
    let mut in_garbage = false;
    iter::from_fn(move || {
        while let Some(byte) = bytes.next() {
            match byte {
                b'!' if in_garbage => {
                    bytes.next();
                }
                b'>' if in_garbage => in_garbage = false,
                _ if in_garbage => {}
                b'<' => in_garbage = true,
                b'{' => {
                    depth += 1;
                    return Some(depth);
                }
                b'}' => depth -= 1,
                b',' => {}
                _ => unreachable!(),
            }
        }
        None
    })
    .sum()
}

pub fn part_2(input: &str) -> usize {
    let mut bytes = input.bytes();
    let mut in_garbage = false;
    iter::from_fn(move || {
        while let Some(byte) = bytes.next() {
            match byte {
                b'!' if in_garbage => {
                    bytes.next();
                }
                b'>' if in_garbage => in_garbage = false,
                _ if in_garbage => return Some(()),
                b'<' => in_garbage = true,
                b'{' | b'}' | b',' => {}
                _ => unreachable!(),
            }
        }
        None
    })
    .count()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_09.txt");

    #[test_case("{}" => 1)]
    #[test_case("{{{}}}" => 6)]
    #[test_case("{{},{}}" => 5)]
    #[test_case("{{{},{},{{}}}}" => 16)]
    #[test_case("{<a>,<a>,<a>,<a>}" => 1)]
    #[test_case("{{<ab>},{<ab>},{<ab>},{<ab>}}" => 9)]
    #[test_case("{{<!!>},{<!!>},{<!!>},{<!!>}}" => 9)]
    #[test_case("{{<a!>},{<a!>},{<a!>},{<ab>}}" => 3)]
    #[test_case(INPUT => 11089)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case("<>" => 0)]
    #[test_case("<random characters>" => 17)]
    #[test_case("<<<<>" => 3)]
    #[test_case("<{!>}>" => 2)]
    #[test_case("<!!>" => 0)]
    #[test_case("<!!!>>" => 0)]
    #[test_case("<{o\"i!a,<{i<a>" => 10)]
    #[test_case(INPUT => 5288)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
