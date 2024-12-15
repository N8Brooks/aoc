use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    let (map, moves) = input.split_once("\n\n").unwrap();
    let mut map = map
        .lines()
        .map(|line| line.bytes().collect_vec())
        .collect_vec();
    let moves = moves.replace("\n", "");
    let mut pos_1 = map
        .iter()
        .enumerate()
        .find_map(|(i, line)| line.iter().position(|&c| c == b'@').map(|j| (i, j)))
        .unwrap();
    for move_x in moves.bytes() {
        let move_ = match move_x {
            b'^' => |(i, j)| (i - 1, j),
            b'v' => |(i, j)| (i + 1, j),
            b'<' => |(i, j)| (i, j - 1),
            b'>' => |(i, j)| (i, j + 1),
            _ => unreachable!(),
        };
        let pos_2 = move_(pos_1);
        map[pos_1.0][pos_1.1] = b'.';
        match map[pos_2.0][pos_2.1] {
            b'.' | b'@' => pos_1 = pos_2,
            b'O' => {
                let mut pos_3 = pos_2;
                while map[pos_3.0][pos_3.1] != b'#' {
                    match map[pos_3.0][pos_3.1] {
                        b'.' | b'@' => {
                            map[pos_2.0][pos_2.1] = b'.';
                            map[pos_3.0][pos_3.1] = b'O';
                            pos_1 = pos_2;
                            break;
                        }
                        b'O' => {}
                        b'#' => {
                            break;
                        }
                        _ => unreachable!(),
                    }
                    pos_3 = move_(pos_3);
                }
            }
            b'#' => {}
            _ => unreachable!(),
        };
        map[pos_1.0][pos_1.1] = b'@';
    }
    map.into_iter()
        .enumerate()
        .flat_map(|(i, line)| line.into_iter().enumerate().map(move |(j, c)| (i, j, c)))
        .filter(|(_, _, c)| *c == b'O')
        .map(|(i, j, _)| 100 * i + j)
        .sum()
}

pub fn part_2(input: &str) -> usize {
    let (map, moves) = input.split_once("\n\n").unwrap();
    let moves = moves.replace("\n", "");
    let (mut i1, mut j1) = map
        .lines()
        .enumerate()
        .find_map(|(i, line)| line.bytes().position(|c| c == b'@').map(|j| (i, 2 * j)))
        .unwrap();
    let mut map = map
        .lines()
        .map(|line| {
            line.bytes()
                .flat_map(|b| match b {
                    b'@' => b"@.",
                    b'O' => b"[]",
                    b'#' => b"##",
                    b'.' => b"..",
                    _ => unreachable!(),
                })
                .copied()
                .collect_vec()
        })
        .collect_vec();
    for move_x in moves.bytes() {
        let move_ = match move_x {
            b'^' => |(i, j)| (i - 1, j),
            b'v' => |(i, j)| (i + 1, j),
            b'<' => |(i, j)| (i, j - 1),
            b'>' => |(i, j)| (i, j + 1),
            _ => unreachable!(),
        };
        let pos_2 = move_((i1, j1));
        map[i1][j1] = b'.';
        match map[pos_2.0][pos_2.1] {
            b'.' | b'@' => (i1, j1) = pos_2,
            b'[' | b']' if b"<>".contains(&move_x) => {
                let mut pos_3 = pos_2;
                while map[pos_3.0][pos_3.1] != b'#' {
                    match map[pos_3.0][pos_3.1] {
                        b'.' | b'@' => {
                            map[pos_2.0][pos_2.1] = b'.';
                            let range = if move_x == b'<' {
                                pos_3.1..pos_2.1
                            } else {
                                pos_2.1 + 1..pos_3.1 + 1
                            };
                            range.zip(b"[]".iter().cycle()).for_each(|(j, &b)| {
                                map[pos_3.0][j] = b;
                            });
                            (i1, j1) = pos_2;
                            break;
                        }
                        b'[' | b']' => {}
                        b'#' => {
                            break;
                        }
                        b => {
                            println!("{}", String::from_utf8_lossy(&[b]));
                            unreachable!()
                        }
                    }
                    pos_3 = move_(pos_3);
                }
            }
            b @ (b'[' | b']') => {
                let mut boxes_1 = vec![
                    (pos_2, b),
                    if b == b'[' {
                        ((pos_2.0, pos_2.1 + 1), b']')
                    } else {
                        ((pos_2.0, pos_2.1 - 1), b'[')
                    },
                ];
                let mut idx = 0;
                let mut success = true;
                while idx < boxes_1.len() {
                    let ((i1, j1), _) = boxes_1[idx];
                    let (i2, j2) = move_((i1, j1));
                    match map[i2][j2] {
                        b'.' | b'@' => {}
                        b @ (b'[' | b']') => {
                            boxes_1.extend(&[
                                ((i2, j2), b),
                                if b == b'[' {
                                    ((i2, j2 + 1), b']')
                                } else {
                                    ((i2, j2 - 1), b'[')
                                },
                            ]);
                        }
                        b'#' => {
                            success = false;
                            break;
                        }
                        _ => unreachable!(),
                    }
                    idx += 1;
                }
                if success {
                    for &((i1, j1), _) in &boxes_1 {
                        map[i1][j1] = b'.';
                    }
                    for &(pos_1, b) in &boxes_1 {
                        let (i2, j2) = move_(pos_1);
                        map[i2][j2] = b;
                    }
                    (i1, j1) = pos_2;
                }
            }
            b'#' => {}
            _ => unreachable!(),
        };
        map[i1][j1] = b'@';
    }
    map.into_iter()
        .enumerate()
        .flat_map(|(i, line)| line.into_iter().enumerate().map(move |(j, c)| (i, j, c)))
        .filter(|(_, _, c)| *c == b'[')
        .map(|(i, j, _)| 100 * i + j)
        .sum()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_15.txt");

    const SMALL_EXAMPLE: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const LARGE_EXAMPLE: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test_case(SMALL_EXAMPLE, 2028)]
    #[test_case(LARGE_EXAMPLE, 10092)]
    #[test_case(INPUT, 1360570)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    const THIS_SITUATION: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    #[test_case(THIS_SITUATION, 618)]
    #[test_case(LARGE_EXAMPLE, 9021)]
    #[test_case(INPUT, 1381446)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
