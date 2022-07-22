use std::iter::Iterator;

fn hit_trees(input: &[u8], slopes: &[(usize, usize)]) -> u32 {
    let width = input.iter().position(|&c| c == b'\n').unwrap();
    slopes
        .iter()
        .map(|(xd, yd)| {
            let mut trees = 0;
            let (mut x, mut y) = (0, 0);
            let mut i = 0;
            while i < input.len() {
                if input[i] == b'#' {
                    trees += 1;
                }
                (x, y) = (x + xd, y + yd);
                i = y * (width + 1) + x % width;
            }
            println!("i {trees}");
            trees
        })
        .product()
}

pub fn part_1(input: &str) -> u32 {
    hit_trees(input.as_bytes(), &[(3, 1)])
}

pub fn part_2(input: &str) -> u32 {
    hit_trees(input.as_bytes(), &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)])
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};
    use lazy_static::lazy_static;
    use std::fs::read_to_string;

    static EXAMPLE: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    lazy_static! {
        static ref INPUT: String = read_to_string("src/year_2020/testdata/day_03.txt").unwrap();
    }

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE), 7);
    }

    #[test]
    fn part_1_input() {
        assert_eq!(part_1(&INPUT), 207);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE), 336);
    }

    #[test]
    fn part_2_input() {
        assert_eq!(part_2(&INPUT), 2655892800);
    }
}
