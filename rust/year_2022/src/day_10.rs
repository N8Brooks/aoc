use itertools::Itertools as _;
use std::iter::repeat_n;

pub fn part_1(input: &str) -> isize {
    iter_cycles(input)
        .enumerate()
        .skip(19)
        .step_by(40)
        .map(|(i, x)| (i + 1).cast_signed() * x)
        .sum()
}

pub fn part_2(input: &str) -> String {
    let rows = crt_rows(input);
    transpose(rows)
        .as_chunks()
        .0
        .iter()
        .map(|&cols| match transpose(cols).each_ref() {
            #[rustfmt::skip]
            [
                b".##..",
                b"#..#.",
                b"#..#.",
                b"####.",
                b"#..#.",
                b"#..#.",
            ] => 'A',
            #[rustfmt::skip]
            [
                b"###..",
                b"#..#.",
                b"###..",
                b"#..#.",
                b"#..#.",
                b"###..",
            ] => 'B',
            #[rustfmt::skip]
            [
                b"..##.",
                b"...#.",
                b"...#.",
                b"...#.",
                b"#..#.",
                b".##..",
            ] => 'J',
            #[rustfmt::skip]
            [
                b"#..#.",
                b"#.#..",
                b"##...",
                b"#.#..",
                b"#.#..",
                b"#..#.",
            ] => 'K',
            #[rustfmt::skip]
            [
                b"#....",
                b"#....",
                b"#....",
                b"#....",
                b"#....",
                b"####.",
            ] => 'L',
            #[rustfmt::skip]
            [
                b"###..",
                b"#..#.",
                b"#..#.",
                b"###..",
                b"#....",
                b"#....",
            ] => 'P',
            #[rustfmt::skip]
            [
                b"###..",
                b"#..#.",
                b"#..#.",
                b"###..",
                b"#.#..",
                b"#..#.",
            ] => 'R',
            rows => panic!(
                "unknown letter:\n{}",
                rows.map(|row| str::from_utf8(row).unwrap()).join("\n")
            ),
        })
        .collect()
}

fn iter_cycles(input: &str) -> impl Iterator<Item = isize> {
    let mut x = 1;
    parse_instructions(input).flat_map(move |instruction| {
        if let Some(dx) = instruction {
            let res = repeat_n(x, 2);
            x += dx;
            res
        } else {
            repeat_n(x, 1)
        }
    })
}

fn parse_instructions(input: &str) -> impl Iterator<Item = Option<isize>> {
    input.lines().map(|line| {
        if let Some(dx) = line.strip_prefix("addx ") {
            Some(dx.parse().unwrap())
        } else if line == "noop" {
            None
        } else {
            panic!("invalid instruction: {line}")
        }
    })
}

fn crt_rows(input: &str) -> [[u8; 40]; 6] {
    iter_cycles(input)
        .enumerate()
        .map(|(i, x)| {
            let pos = (i % 40).cast_signed();
            if x.abs_diff(pos) > 1 { b'.' } else { b'#' }
        })
        .array_chunks()
        .collect_array()
        .unwrap()
}

/// Transposes a 2D array of size MxN into one of size NxM.
fn transpose<const M: usize, const N: usize, T>(m: [[T; N]; M]) -> [[T; M]; N] {
    use std::array::from_fn;
    let mut iters = m.map(|r| r.into_iter());
    from_fn(|_| from_fn(|i| iters[i].next().unwrap()))
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

    const INPUT: &str = include_str!("../test_data/day_10.txt");

    #[test_case(EXAMPLE => 13140)]
    #[test_case(INPUT => 15120)]
    fn part_1(input: &str) -> isize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => [
        *b"##..##..##..##..##..##..##..##..##..##..",
        *b"###...###...###...###...###...###...###.",
        *b"####....####....####....####....####....",
        *b"#####.....#####.....#####.....#####.....",
        *b"######......######......######......####",
        *b"#######.......#######.......#######.....",
    ])]
    fn example_2(input: &str) -> [[u8; 40]; 6] {
        super::crt_rows(input)
    }

    #[test_case(INPUT => "RKPJBPLA")]
    fn part_2(input: &str) -> String {
        super::part_2(input)
    }
}
