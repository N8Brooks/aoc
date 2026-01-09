#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Square {
    Sand,
    Clay,
    Water,
    Flowing,
}

use Square::*;

pub fn part_1(input: &str) -> usize {
    parse_grid(input)
        .into_iter()
        .flatten()
        .filter(|&sq| sq == Water || sq == Flowing)
        .count()
}

pub fn part_2(input: &str) -> usize {
    parse_grid(input)
        .into_iter()
        .flatten()
        .filter(|&sq| sq == Water)
        .count()
}

fn parse_grid(input: &str) -> Vec<[Square; 1000]> {
    let veins: Vec<_> = parse_input(input).collect();

    let min_y = *veins
        .iter()
        .map(|vein| match vein {
            Horizontal { y, .. } => y,
            Vertical { y: (y1, _), .. } => y1,
        })
        .min()
        .unwrap();

    let max_y = veins
        .iter()
        .map(|vein| match vein {
            Horizontal { y, .. } => y,
            Vertical { y: (_, y2), .. } => y2,
        })
        .max()
        .unwrap();

    let mut grid = vec![[Sand; 1000]; max_y - min_y + 1];
    for vein in veins {
        match vein {
            Vertical { x, y: (y1, y2) } => {
                grid[y1 - min_y..=y2 - min_y]
                    .iter_mut()
                    .for_each(|row| row[x] = Clay);
            }
            Horizontal { x: (x1, x2), y } => {
                grid[y - min_y][x1..=x2].fill(Clay);
            }
        }
    }

    let mut stack = vec![(0, 500)];
    while let Some((y, x)) = stack.pop() {
        let Ok([row_1, row_2]) = grid.get_disjoint_mut([y, y + 1]) else {
            grid[y][x] = Flowing;
            continue;
        };
        match row_2[x] {
            Sand => {
                row_1[x] = Flowing;
                stack.extend([(y, x), (y + 1, x)]);
            }
            Clay | Water => {
                let mut left = x;
                let can_flow_left = loop {
                    match (row_1[left - 1], row_2[left]) {
                        (Clay, _) => break false,
                        (_, Sand | Flowing) => {
                            break true;
                        }
                        _ => left -= 1,
                    }
                };

                let mut right = x;
                let can_flow_right = loop {
                    match (row_1[right + 1], row_2[right]) {
                        (Clay, _) => break false,
                        (_, Sand | Flowing) => {
                            break true;
                        }
                        _ => right += 1,
                    }
                };

                let sq = if can_flow_left || can_flow_right {
                    Flowing
                } else {
                    Water
                };

                row_1[left..=right].fill(sq);

                if can_flow_left {
                    stack.push((y, left));
                }
                if can_flow_right {
                    stack.push((y, right));
                }
            }
            Flowing => {
                row_1[x] = Flowing;
            }
        }
    }

    grid
}

#[derive(Debug)]
enum Vein {
    Horizontal { x: (usize, usize), y: usize },
    Vertical { x: usize, y: (usize, usize) },
}

use Vein::*;

fn parse_input(input: &str) -> impl Iterator<Item = Vein> {
    input.lines().map(|line| {
        let (one, two) = line.split_once(", ").unwrap();
        let (axis_1, idx_1) = one.split_once('=').unwrap();
        let idx_1: usize = idx_1.parse().unwrap();
        let (axis_2, range) = two.split_once('=').unwrap();
        let (start, end) = range.split_once("..").unwrap();
        let start: usize = start.parse().unwrap();
        let end: usize = end.parse().unwrap();

        match (axis_1, axis_2) {
            ("x", "y") => Vein::Vertical {
                x: idx_1,
                y: (start, end),
            },
            ("y", "x") => Vein::Horizontal {
                x: (start, end),
                y: idx_1,
            },
            _ => panic!("invalid axes {axis_1}, {axis_2}"),
        }
    })
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_17.txt");

    const EXAMPLE_1: &str = "x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504";

    #[test_case(EXAMPLE_1 => 57)]
    #[test_case(INPUT => 33724)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE_1 => 29)]
    #[test_case(INPUT => 27420)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
