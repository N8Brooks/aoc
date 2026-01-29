use std::iter::once;

use hashbrown::HashSet;

pub fn part_1(input: &str) -> usize {
    let (mut i, mut j) = (0, 0);
    parse_input(input)
        .map(|(dx, dy)| {
            i += dx;
            j += dy;
            (i, j)
        })
        .chain(once((0, 0)))
        .collect::<HashSet<_>>()
        .len()
}

pub fn part_2(input: &str) -> usize {
    let (mut i1, mut j1) = (0, 0);
    let (mut i2, mut j2) = (0, 0);
    parse_input(input)
        .enumerate()
        .map(|(k, (dx, dy))| {
            if k & 1 == 0 {
                i1 += dx;
                j1 += dy;
                (i1, j1)
            } else {
                i2 += dx;
                j2 += dy;
                (i2, j2)
            }
        })
        .chain(once((0, 0)))
        .collect::<HashSet<_>>()
        .len()
}

fn parse_input(input: &str) -> std::iter::Map<std::str::Bytes<'_>, impl FnMut(u8) -> (i32, i32)> {
    input.bytes().map(|b| match b {
        b'^' | b'v' => (0, if b == b'^' { 1 } else { -1 }),
        b'>' | b'<' => (if b == b'>' { 1 } else { -1 }, 0),
        _ => (0, 0),
    })
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_03.txt");

    #[test_case(">" => 2)]
    #[test_case("^>v<" => 4)]
    #[test_case("^v^v^v^v^v" => 2)]
    #[test_case(INPUT => 2081)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case("^v" => 3; "example 1")]
    #[test_case("^>v<" => 3; "example 2")]
    #[test_case("^v^v^v^v^v" => 11)]
    #[test_case(INPUT => 2341)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
