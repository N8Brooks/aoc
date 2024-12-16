use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    let (map, moves) = input.split_once("\n\n").unwrap();
    let mut map = map
        .lines()
        .map(|line| line.bytes().collect_vec())
        .collect_vec();
    let (mut i1, mut j1) = map
        .iter()
        .enumerate()
        .find_map(|(i, line)| line.iter().position(|&c| c == b'@').map(|j| (i, j)))
        .unwrap();
    map[i1][j1] = b'.';
    for move_ in moves.bytes().filter(|&c| c != b'\n').map(mover) {
        let (i2, j2) = move_(i1, j1);
        match map[i2][j2] {
            b'.' => (i1, j1) = (i2, j2),
            b'O' => {
                let (mut i3, mut j3) = (i2, j2);
                while map[i3][j3] == b'O' {
                    (i3, j3) = move_(i3, j3);
                }
                if map[i3][j3] == b'.' {
                    map[i2][j2] = b'.';
                    map[i3][j3] = b'O';
                    (i1, j1) = (i2, j2);
                }
            }
            b'#' => {}
            b2 => panic!("unexpected char: {}", b2 as char),
        };
    }
    map.into_iter()
        .enumerate()
        .flat_map(|(i, line)| line.into_iter().enumerate().map(move |(j, c)| (i, j, c)))
        .filter(|(_, _, c)| *c == b'O')
        .map(|(i, j, _)| 100 * i + j)
        .sum()
}

pub fn part_2(input: &str) -> usize {
    fn complete_box(i: usize, j: usize, b: u8) -> [(usize, usize, u8); 2] {
        match b {
            b'[' => [(i, j, b'['), (i, j + 1, b']')],
            b']' => [(i, j - 1, b'['), (i, j, b']')],
            _ => panic!("unexpected box: {}", b as char),
        }
    }

    let (map, moves) = input.split_once("\n\n").unwrap();
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
                    b'.' | b'@' => b"..",
                    b'O' => b"[]",
                    b'#' => b"##",
                    b => panic!("unexpected char: {}", b as char),
                })
                .copied()
                .collect_vec()
        })
        .collect_vec();

    for dir in moves.bytes().filter(|&c| c != b'\n') {
        let move_ = mover(dir);
        let (i2, j2) = move_(i1, j1);
        match map[i2][j2] {
            b'.' => (i1, j1) = (i2, j2),
            b'[' | b']' if dir == b'<' || dir == b'>' => {
                let (mut i3, mut j3) = (i2, j2);
                while b"[]".contains(&map[i3][j3]) {
                    (i3, j3) = move_(i3, j3);
                }
                if map[i3][j3] == b'.' {
                    if dir == b'<' {
                        map[i3].copy_within(j3 + 1..j2 + 1, j3)
                    } else {
                        map[i3].copy_within(j2..j3, j2 + 1)
                    };
                    map[i2][j2] = b'.';
                    (i1, j1) = (i2, j2);
                }
            }
            b2 @ (b'[' | b']') => {
                let mut boxes = Vec::from(complete_box(i2, j2, b2));
                let mut idx = 0;
                let mut success = true;
                while idx < boxes.len() {
                    let (i, j, _) = boxes[idx];
                    let (i, j) = move_(i, j);
                    match map[i][j] {
                        b'.' => {}
                        b @ (b'[' | b']') => boxes.extend(complete_box(i, j, b)),
                        b'#' => {
                            success = false;
                            break;
                        }
                        b => panic!("unexpected char: {}", b as char),
                    }
                    idx += 1;
                }
                if success {
                    for (i, j, b) in boxes.into_iter().rev() {
                        map[i][j] = b'.';
                        let (i, j) = move_(i, j);
                        map[i][j] = b;
                    }
                    (i1, j1) = (i2, j2);
                }
            }
            b'#' => {}
            b2 => panic!("unexpected char: {}", b2 as char),
        };
    }

    map.into_iter()
        .enumerate()
        .flat_map(|(i, line)| line.into_iter().enumerate().map(move |(j, c)| (i, j, c)))
        .filter(|(_, _, c)| *c == b'[')
        .map(|(i, j, _)| 100 * i + j)
        .sum()
}

fn mover(direction: u8) -> fn(usize, usize) -> (usize, usize) {
    match direction {
        b'^' => |i, j| (i - 1, j),
        b'v' => |i, j| (i + 1, j),
        b'<' => |i, j| (i, j - 1),
        b'>' => |i, j| (i, j + 1),
        _ => panic!("unexpected direction: {}", direction as char),
    }
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
