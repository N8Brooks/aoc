use std::collections::HashSet;

fn parse_cave(input: &str) -> HashSet<(isize, isize)> {
    let mut cave = HashSet::new();
    for line in input.lines() {
        let mut coordinates = line.split(" -> ").map(|coordinate| {
            let (x, y) = coordinate.split_once(',').unwrap();
            (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap())
        });
        let (mut x, mut y) = coordinates.next().unwrap();
        for (x_end, y_end) in coordinates {
            let x_d = (x_end - x).signum();
            let y_d = (y_end - y).signum();
            while (x, y) != (x_end, y_end) {
                cave.insert((x, y));
                x += x_d;
                y += y_d;
            }
            cave.insert((x, y));
        }
    }
    cave
}

pub fn part_1(input: &str) -> usize {
    let mut cave = parse_cave(input);
    let max_y = *cave.iter().map(|(_x, y)| y).max().unwrap();
    let rock_count = cave.len();
    let mut stack = vec![(500, 0)];
    while let Some((x_0, y_0)) = stack.pop() {
        if y_0 == max_y {
            return cave.len() - rock_count;
        }
        let y_1 = y_0 + 1;
        'search: {
            for x_1 in [x_0, x_0 - 1, x_0 + 1] {
                if cave.contains(&(x_1, y_1)) {
                    continue;
                }
                stack.extend([(x_0, y_0), (x_1, y_1)]);
                break 'search;
            }
            cave.insert((x_0, y_0));
        }
    }
    unreachable!();
}

pub fn part_2(input: &str) -> usize {
    let mut cave = parse_cave(input);
    let max_y = *cave.iter().map(|(_x, y)| y).max().unwrap() + 1;
    let rock_count = cave.len();
    let mut stack = vec![(500, 0)];
    cave.insert((500, 0));
    while let Some((x, y)) = stack.pop() {
        if y == max_y {
            continue;
        }
        for coordinate in [(x - 1, y + 1), (x, y + 1), (x + 1, y + 1)] {
            if cave.contains(&coordinate) {
                continue;
            }
            cave.insert(coordinate);
            stack.push(coordinate);
        }
    }
    cave.len() - rock_count
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    const INPUT: &str = include_str!("../../../testdata/year_2022/day_14.txt");

    #[test_case(EXAMPLE, 24)]
    #[test_case(INPUT, 1133)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 93)]
    #[test_case(INPUT, 27566)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
