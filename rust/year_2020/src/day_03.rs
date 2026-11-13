pub fn part_1(input: &str) -> usize {
    count_collisions(input, (1, 3))
}

pub fn part_2(input: &str) -> usize {
    [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
        .into_iter()
        .map(|slope| count_collisions(input, slope))
        .product()
}

fn count_collisions(input: &str, (di, dj): (usize, usize)) -> usize {
    input
        .lines()
        .step_by(di)
        .enumerate()
        .skip(1)
        .filter(|&(k, line)| line.as_bytes()[k * dj % line.len()] == b'#')
        .count()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_03.txt");

    const EXAMPLE: &str = "\
..##.........##.........##.........##.........##.........##.......
#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
.#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
.#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....
.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
.#........#.#........#.#........#.#........#.#........#.#........#
#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
#...##....##...##....##...##....##...##....##...##....##...##....#
.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#";

    #[test_case(EXAMPLE => 7)]
    #[test_case(INPUT => 207)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 336)]
    #[test_case(INPUT => 2655892800)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
