use itertools::{izip, Itertools as _};

pub fn part_1(input: &str) -> usize {
    let mat = input.lines().map(|line| line.as_bytes()).collect_vec();
    let m = mat.len();
    let n = mat[0].len();
    (0..m)
        .cartesian_product(0..n)
        .cartesian_product([
            [-1, -1],
            [-1, 0],
            [-1, 1],
            [0, -1],
            [0, 1],
            [1, -1],
            [1, 0],
            [1, 1],
        ])
        .filter(|&((i, j), [di, dj])| {
            "XMAS".bytes().enumerate().all(|(k, c)| {
                let i = if let Some(i) = i.checked_add_signed(di * k as isize) {
                    i
                } else {
                    return false;
                };
                let j = if let Some(j) = j.checked_add_signed(dj * k as isize) {
                    j
                } else {
                    return false;
                };
                mat.get(i)
                    .is_some_and(|line| line.get(j).is_some_and(|&x| x == c))
            })
        })
        .count()
}

pub fn part_2(input: &str) -> usize {
    input
        .lines()
        .tuple_windows()
        .flat_map(|(a, b, c)| {
            izip!(a.bytes(), b.bytes(), c.bytes())
                .tuple_windows()
                .filter(|(a, b, c)| {
                    let major = [a.0, c.2];
                    let minor = [a.2, c.0];
                    b.1 == b'A'
                        && major.contains(&b'M')
                        && major.contains(&b'S')
                        && minor.contains(&b'M')
                        && minor.contains(&b'S')
                })
        })
        .count()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_04.txt");

    const EXAMPLE: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    // 2647 - too low
    #[test_case(EXAMPLE, 18)]
    #[test_case(INPUT, 2654)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 9)]
    #[test_case(INPUT, 1990)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
