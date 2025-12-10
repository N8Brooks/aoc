pub fn part_1(input: &str) -> usize {
    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.bytes().map(|b| b == b'@').collect())
        .collect();
    let (m, n) = (grid.len(), grid[0].len());

    grid.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, b)| *b)
                .map(move |(j, _)| (i, j))
        })
        .map(|(i, j)| neighbors(m, n, i, j).filter(|&(i, j)| grid[i][j]).count())
        .filter(|&count| count < 4)
        .count()
}

pub fn part_2(input: &str) -> usize {
    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.bytes().map(|b| b == b'@').collect())
        .collect();
    let (m, n) = (grid.len(), grid[0].len());

    let mut counts: Vec<Vec<_>> = grid
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, &b)| {
                    b.then(|| neighbors(m, n, i, j).filter(|&(i, j)| grid[i][j]).count())
                })
                .collect()
        })
        .collect();

    let mut stack: Vec<_> = counts
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, count)| count.is_some_and(|count| count < 4))
                .map(move |(j, _)| (i, j))
        })
        .collect();

    let mut rolls_of_paper = 0;
    while let Some((i, j)) = stack.pop() {
        let update = neighbors(m, n, i, j).filter(|&(ni, nj)| {
            counts[ni][nj].as_mut().is_some_and(|count| {
                *count -= 1;
                *count == 3
            })
        });
        stack.extend(update);
        rolls_of_paper += 1;
    }
    rolls_of_paper
}

fn neighbors(m: usize, n: usize, i: usize, j: usize) -> impl Iterator<Item = (usize, usize)> {
    let im1 = i.checked_sub(1);
    let ip1 = (i + 1 < m).then_some(i + 1);
    let jm1 = j.checked_sub(1);
    let jp1 = (j + 1 < n).then_some(j + 1);
    [
        (im1, jm1),
        (im1, Some(j)),
        (im1, jp1),
        (Some(i), jm1),
        (Some(i), jp1),
        (ip1, jm1),
        (ip1, Some(j)),
        (ip1, jp1),
    ]
    .into_iter()
    .filter_map(|(i, j)| i.zip(j))
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_04.txt");

    const EXAMPLE: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test_case(EXAMPLE => 13)]
    #[test_case(INPUT => 1553)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 43)]
    #[test_case(INPUT => 8442)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
