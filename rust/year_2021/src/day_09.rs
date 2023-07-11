use itertools::Itertools;

pub fn part_1(input: &str) -> usize {
    let input = input.lines().map(|line| line.as_bytes()).collect_vec();
    let m = input.len();
    let n = input[0].len();
    input
        .iter()
        .enumerate()
        .flat_map(|(i, line)| line.iter().enumerate().map(move |(j, byte)| (i, j, byte)))
        .filter_map(|(i, j, &byte)| {
            [
                i.checked_sub(1).map(|i| (i, j)),
                (i + 1 < m).then_some((i + 1, j)),
                j.checked_sub(1).map(|j| (i, j)),
                (j + 1 < n).then_some((i, j + 1)),
            ]
            .into_iter()
            .flatten()
            .all(|(i, j)| input[i][j] > byte)
            .then(|| usize::from(byte - b'0' + 1))
        })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    let mut input = input
        .lines()
        .map(|line| line.bytes().collect_vec())
        .collect_vec();
    let m = input.len();
    let n = input[0].len();
    (0..m)
        .cartesian_product(0..n)
        .filter_map(|(i, j)| {
            if input[i][j] == b'9' {
                return None;
            }
            let mut stack = vec![(i, j)];
            input[i][j] = b'9';
            let mut count = 1;
            while let Some((i, j)) = stack.pop() {
                let neighbors = [
                    i.checked_sub(1).map(|i| (i, j)),
                    (i + 1 < m).then_some((i + 1, j)),
                    j.checked_sub(1).map(|j| (i, j)),
                    (j + 1 < n).then_some((i, j + 1)),
                ]
                .into_iter()
                .flatten()
                .filter(|&(i, j)| input[i][j] != b'9')
                .collect_vec();
                count += neighbors.len();
                for &(i, j) in &neighbors {
                    input[i][j] = b'9';
                }
                stack.extend(neighbors);
            }
            Some(count)
        })
        .sorted()
        .rev()
        .take(3)
        .product()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    const INPUT: &str = include_str!("../../../testdata/year_2021/day_09.txt");

    #[test_case(EXAMPLE, 15)]
    #[test_case(INPUT, 500)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 1134)]
    #[test_case(INPUT, 970200)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
