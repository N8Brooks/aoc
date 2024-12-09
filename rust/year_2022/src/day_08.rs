use itertools::Itertools;

pub fn part_1(input: &str) -> usize {
    let tree_heights: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.bytes().map(|height| height - b'0').collect())
        .collect();
    let mut visible = vec![vec![false; tree_heights.len()]; tree_heights[0].len()];
    for (i, heights) in tree_heights.iter().enumerate() {
        let mut tallest_tree = 0;
        for (j, &height) in heights.iter().enumerate() {
            if height >= tallest_tree {
                tallest_tree = height + 1;
                visible[i][j] = true;
            }
        }
        let mut tallest_tree = 0;
        for (j, &height) in heights.iter().enumerate().rev() {
            if height >= tallest_tree {
                tallest_tree = height + 1;
                visible[i][j] = true;
            }
        }
    }
    for j in 0..tree_heights[0].len() {
        let mut tallest_tree = 0;
        for i in 0..tree_heights.len() {
            let height = tree_heights[i][j];
            if height >= tallest_tree {
                tallest_tree = height + 1;
                visible[i][j] = true;
            }
        }
        let mut tallest_tree = 0;
        for i in (0..tree_heights.len()).rev() {
            let height = tree_heights[i][j];
            if height >= tallest_tree {
                tallest_tree = height + 1;
                visible[i][j] = true;
            }
        }
    }
    visible
        .iter()
        .flatten()
        .map(|&is_visible| usize::from(is_visible))
        .sum()
}

pub fn part_2(input: &str) -> usize {
    let tree_heights: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.bytes().map(|height| height - b'0').collect())
        .collect();

    let scenic_score = |i: usize, j: usize| -> usize {
        let given_height = tree_heights[i][j];

        let up = {
            let mut iter = (0..i).rev().map(|i| tree_heights[i][j]).peekable();
            let visible_count = iter
                .peeking_take_while(|&height| given_height > height)
                .count();
            let blocking_count = iter.next().iter().count();
            visible_count + blocking_count
        };

        let left = {
            let mut iter = (0..j).rev().map(|j| tree_heights[i][j]).peekable();
            let visible_count = iter
                .peeking_take_while(|&height| given_height > height)
                .count();
            let blocking_count = iter.next().iter().count();
            visible_count + blocking_count
        };

        let right = {
            let mut iter = (j + 1..tree_heights[0].len())
                .map(|j| tree_heights[i][j])
                .peekable();
            let visible_count = iter
                .peeking_take_while(|&height| given_height > height)
                .count();
            let blocking_count = iter.next().iter().count();
            visible_count + blocking_count
        };

        let down = {
            let mut iter = (i + 1..tree_heights.len())
                .map(|i| tree_heights[i][j])
                .peekable();
            let visible_count = iter
                .peeking_take_while(|&height| given_height > height)
                .count();
            let blocking_count = iter.next().iter().count();
            visible_count + blocking_count
        };

        up * left * right * down
    };

    (0..tree_heights.len())
        .flat_map(|i| (0..tree_heights[0].len()).map(move |j| scenic_score(i, j)))
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "30373
25512
65332
33549
35390";

    const INPUT: &str = include_str!("../../../test_data/year_2022/day_08.txt");

    #[test_case(EXAMPLE, 21)]
    #[test_case(INPUT, 1785)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 8)]
    #[test_case(INPUT, 345168)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
