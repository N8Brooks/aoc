use itertools::Itertools as _;

pub fn part_1<const N: usize>(input: &str, n: usize) -> usize {
    let mut free_memory = parse_input(input)
        .take(n)
        .fold([[true; N]; N], |mut memory, (i, j)| {
            memory[i][j] = false;
            memory
        });
    let mut frontier_1 = vec![(0, 0)];
    let mut frontier_2 = Vec::new();
    let mut count = 0;
    while !frontier_1.is_empty() {
        for (i, j) in frontier_1.drain(..) {
            if (i, j) == (N - 1, N - 1) {
                return count;
            }
            for (i, j) in neighbors::<N>(i, j) {
                if free_memory[i][j] {
                    free_memory[i][j] = false;
                    frontier_2.push((i, j));
                }
            }
        }
        std::mem::swap(&mut frontier_1, &mut frontier_2);
        count += 1;
    }
    panic!("no solution found");
}

pub fn part_2<const N: usize>(input: &str) -> String {
    let bytes = parse_input(input).collect_vec();

    let is_escapable = |idx: usize| -> bool {
        let mut free_memory = [[true; N]; N];
        for &(i, j) in &bytes[..=idx] {
            free_memory[i][j] = false;
        }
        let mut stack = vec![(0, 0)];
        while let Some((i, j)) = stack.pop() {
            if (i, j) == (N - 1, N - 1) {
                return true;
            }
            for (i, j) in neighbors::<N>(i, j) {
                if free_memory[i][j] {
                    free_memory[i][j] = false;
                    stack.push((i, j));
                }
            }
        }
        false
    };

    let (mut l, mut r) = (0, bytes.len());
    while l < r {
        let m = (l + r) / 2;
        if is_escapable(m) {
            l = m + 1;
        } else {
            r = m;
        }
    }
    let (i, j) = bytes[l]; // assumes some escapable
    format!("{j},{i}")
}

fn parse_input(input: &str) -> impl Iterator<Item = (usize, usize)> + '_ {
    input.lines().map(|line| {
        let (j, i) = line.split_once(",").unwrap();
        (i.parse().unwrap(), j.parse().unwrap())
    })
}

fn neighbors<const N: usize>(i: usize, j: usize) -> impl Iterator<Item = (usize, usize)> {
    [
        i.checked_sub(1).zip(Some(j)),
        Some(i).zip(j.checked_sub(1)),
        (i + 1 < N).then_some((i + 1, j)),
        (j + 1 < N).then_some((i, j + 1)),
    ]
    .into_iter()
    .flatten()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_18.txt");

    const EXAMPLE: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test_case(EXAMPLE, 12, 22)]
    fn part_1_example(input: &str, n: usize, expected: usize) {
        assert_eq!(super::part_1::<7>(input, n), expected);
    }

    #[test_case(INPUT, 1024, 268)]
    fn part_1_input(input: &str, n: usize, expected: usize) {
        assert_eq!(super::part_1::<71>(input, n), expected);
    }

    #[test_case(EXAMPLE, "6,1")]
    fn part_2_example(input: &str, expected: &str) {
        assert_eq!(super::part_2::<7>(input), expected);
    }

    #[test_case(INPUT, "64,11")]
    fn part_2_input(input: &str, expected: &str) {
        assert_eq!(super::part_2::<71>(input), expected);
    }
}
