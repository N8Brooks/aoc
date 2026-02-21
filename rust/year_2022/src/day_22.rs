use std::{cell::LazyCell, iter::successors};

use regex::Regex;

#[derive(Debug)]
enum Path {
    Left,
    Right,
    Distance(usize),
}

use Path::*;

pub fn part_1(input: &str) -> usize {
    let (board, path) = parse_input(input);
    let n = board.len() as isize;
    let m = board.iter().map(|row| row.len()).max().unwrap() as isize;
    let (mut di, mut dj) = (0, 1);
    let (mut i, mut j) = start_tile(&board);
    for p in path {
        match p {
            Left => (di, dj) = (-dj, di),
            Right => (di, dj) = (dj, -di),
            Distance(distance) => {
                (i, j) = successors(Some((i, j)), |&(i, j)| {
                    let i = (i as isize + di).rem_euclid(n) as usize;
                    let j = (j as isize + dj).rem_euclid(m) as usize;
                    Some((i, j))
                })
                .filter_map(|(i, j)| Some(((i, j), board[i].get(j).flatten_ref()?)))
                .take(distance + 1)
                .take_while(|&(_, &open)| open)
                .last()
                .unwrap()
                .0;
            }
        }
    }
    final_password((i, j), (di, dj))
}

fn parse_input(input: &str) -> (Vec<Vec<Option<bool>>>, Vec<Path>) {
    let re = LazyCell::new(|| Regex::new(r"(?:L|R|\d+)").unwrap());
    let (board, path) = input.split_once("\n\n").unwrap();
    let board = board
        .lines()
        .map(|line| {
            line.bytes()
                .map(|byte| match byte {
                    b' ' => None,
                    b'#' => Some(false),
                    b'.' => Some(true),
                    _ => panic!("unsupported byte {byte}"),
                })
                .collect()
        })
        .collect();
    let path = re
        .captures_iter(path)
        .map(|token| match &token[0] {
            "L" => Left,
            "R" => Right,
            distance => Distance(distance.parse().unwrap()),
        })
        .collect();
    (board, path)
}

fn final_password((i, j): (usize, usize), dir: (isize, isize)) -> usize {
    let (ip1, jp1) = (i + 1, j + 1);
    let facing = match dir {
        (0, 1) => 0,
        (1, 0) => 1,
        (0, -1) => 2,
        (-1, 0) => 3,
        (i, j) => panic!("unknown direction ({i}, {j})"),
    };
    1000 * ip1 + 4 * jp1 + facing
}

fn start_tile(board: &[Vec<Option<bool>>]) -> (usize, usize) {
    board
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find(|&(_, &tile)| tile == Some(true))
                .map(|(j, _)| (i, j))
        })
        .expect("missing starting tile")
}

pub fn part_2(input: &str) -> usize {
    let (board, path) = parse_input(input);
    let cube = CubeNet::build(&board);
    let (i, j) = start_tile(&board);

    let (start_face, (face_row0, face_col0)) = Face::ALL
        .into_iter()
        .find_map(|face| {
            let (row0, col0) = cube.face_origin(face);
            ((row0..row0 + cube.face_size).contains(&i)
                && (col0..col0 + cube.face_size).contains(&j))
            .then_some((face, (row0, col0)))
        })
        .expect("start tile must belong to exactly one face");

    let mut pos = CubePos {
        face: start_face,
        row: i - face_row0,
        col: j - face_col0,
        dir: Dir::Right,
    };

    for step in path {
        match step {
            Left => pos.dir = pos.dir.turn_left(),
            Right => pos.dir = pos.dir.turn_right(),
            Distance(distance) => {
                for _ in 0..distance {
                    let next = cube.step(pos);
                    let (ni, nj) = cube.to_board(next);
                    let open = board[ni][nj].expect("missing tile for cube face");
                    if !open {
                        break;
                    }
                    pos = next;
                }
            }
        }
    }

    let (fi, fj) = cube.to_board(pos);
    final_password((fi, fj), pos.dir.delta())
}

#[derive(Debug)]
struct CubeNet {
    face_size: usize,
    origins: [Option<(usize, usize)>; Face::COUNT],
    transitions: [[EdgeTransition; Dir::COUNT]; Face::COUNT],
}

impl CubeNet {
    fn build(board: &[Vec<Option<bool>>]) -> Self {
        let (face_size, orientations) = compute_orientations(board);
        let mut origins = [None; Face::COUNT];
        let mut face_orientations = [None; Face::COUNT];

        for (&(bi, bj), &orientation) in &orientations {
            let face = Face::from_normal(orientation.normal);
            origins[face.idx()] = Some((bi * face_size, bj * face_size));
            face_orientations[face.idx()] = Some(orientation);
        }

        let mut transitions = [[None; Dir::COUNT]; Face::COUNT];
        for face in Face::ALL {
            let orientation =
                face_orientations[face.idx()].expect("missing orientation for cube face");
            for dir in Dir::ALL {
                let (edge_axis, dir_vec, neighbor_normal) = match dir {
                    Dir::Right => (orientation.down, orientation.right, orientation.right),
                    Dir::Left => (
                        orientation.down,
                        orientation.right.neg(),
                        orientation.right.neg(),
                    ),
                    Dir::Down => (orientation.right, orientation.down, orientation.down),
                    Dir::Up => (
                        orientation.right,
                        orientation.down.neg(),
                        orientation.down.neg(),
                    ),
                };

                let neighbor_face = Face::from_normal(neighbor_normal);
                let neighbor_orientation = face_orientations[neighbor_face.idx()]
                    .expect("missing neighbor orientation for cube face");

                let (axis, sign) = rotation_for_edge(orientation, dir);
                let new_dir_vec = rotate_90(dir_vec, axis, sign);
                let new_dir = neighbor_orientation.dir_from_vec(new_dir_vec);

                let new_edge_axis = rotate_90(edge_axis, axis, sign);
                let (k_axis, k_reversed) = neighbor_orientation.axis_mapping(new_edge_axis);
                let fixed = fixed_edge_coordinate(face_size, new_dir, k_axis);

                transitions[face.idx()][dir.idx()] = Some(EdgeTransition {
                    to_face: neighbor_face,
                    to_dir: new_dir,
                    k_axis,
                    k_reversed,
                    fixed,
                });
            }
        }

        let transitions =
            transitions.map(|edges| edges.map(|edge| edge.expect("incomplete cube transition")));

        Self {
            face_size,
            origins,
            transitions,
        }
    }

    fn face_origin(&self, face: Face) -> (usize, usize) {
        self.origins[face.idx()].expect("missing cube face origin")
    }

    fn to_board(&self, pos: CubePos) -> (usize, usize) {
        let (row0, col0) = self.face_origin(pos.face);
        (row0 + pos.row, col0 + pos.col)
    }

    fn step(&self, pos: CubePos) -> CubePos {
        let size = self.face_size as isize;
        let (dr, dc) = pos.dir.delta();
        let nr = pos.row as isize + dr;
        let nc = pos.col as isize + dc;
        if nr >= 0 && nr < size && nc >= 0 && nc < size {
            return CubePos {
                row: nr as usize,
                col: nc as usize,
                ..pos
            };
        }

        let transition = self.transitions[pos.face.idx()][pos.dir.idx()];
        let mut k = match pos.dir.edge_axis() {
            Axis::Row => pos.row,
            Axis::Col => pos.col,
        };
        if transition.k_reversed {
            k = self.face_size - 1 - k;
        }
        let (row, col) = match transition.k_axis {
            Axis::Row => (k, transition.fixed),
            Axis::Col => (transition.fixed, k),
        };
        CubePos {
            face: transition.to_face,
            row,
            col,
            dir: transition.to_dir,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Axis {
    Row,
    Col,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Face {
    Up,
    Down,
    Left,
    Right,
    Front,
    Back,
}

impl Face {
    const COUNT: usize = 6;
    const ALL: [Self; Self::COUNT] = [
        Self::Up,
        Self::Down,
        Self::Left,
        Self::Right,
        Self::Front,
        Self::Back,
    ];

    const fn idx(self) -> usize {
        match self {
            Self::Up => 0,
            Self::Down => 1,
            Self::Left => 2,
            Self::Right => 3,
            Self::Front => 4,
            Self::Back => 5,
        }
    }

    fn from_normal(normal: Vec3) -> Self {
        match (normal.x, normal.y, normal.z) {
            (0, -1, 0) => Face::Up,
            (0, 1, 0) => Face::Down,
            (-1, 0, 0) => Face::Left,
            (1, 0, 0) => Face::Right,
            (0, 0, 1) => Face::Front,
            (0, 0, -1) => Face::Back,
            _ => panic!("unknown normal: ({}, {}, {})", normal.x, normal.y, normal.z),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Dir {
    Right,
    Left,
    Down,
    Up,
}

impl Dir {
    const COUNT: usize = 4;
    const ALL: [Self; Self::COUNT] = [Self::Right, Self::Left, Self::Down, Self::Up];

    const fn idx(self) -> usize {
        match self {
            Self::Right => 0,
            Self::Left => 1,
            Self::Down => 2,
            Self::Up => 3,
        }
    }

    fn delta(self) -> (isize, isize) {
        match self {
            Dir::Right => (0, 1),
            Dir::Left => (0, -1),
            Dir::Down => (1, 0),
            Dir::Up => (-1, 0),
        }
    }

    fn turn_left(self) -> Self {
        match self {
            Self::Right => Self::Up,
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
        }
    }

    fn turn_right(self) -> Self {
        match self {
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Up => Self::Right,
        }
    }

    fn edge_axis(self) -> Axis {
        match self {
            Self::Right | Self::Left => Axis::Row,
            Self::Down | Self::Up => Axis::Col,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct CubePos {
    face: Face,
    row: usize,
    col: usize,
    dir: Dir,
}

fn face_size(board: &[Vec<Option<bool>>]) -> usize {
    let mut sizes = Vec::new();
    for row in board {
        let mut start = None;
        for (j, tile) in row.iter().enumerate() {
            if tile.is_some() && start.is_none() {
                start = Some(j);
            } else if tile.is_none() && start.is_some() {
                sizes.push(j - start.unwrap());
                start = None;
            }
        }
        if let Some(start) = start {
            sizes.push(row.len() - start);
        }
    }
    sizes.into_iter().reduce(gcd).unwrap_or(0)
}

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

fn compute_orientations(
    board: &[Vec<Option<bool>>],
) -> (usize, hashbrown::HashMap<(usize, usize), Orientation>) {
    let face_size = face_size(board);
    assert!(face_size > 0, "could not infer cube face size");
    let rows = board.len();
    let cols = board.iter().map(|row| row.len()).max().unwrap_or(0);
    let block_rows = rows / face_size;
    let block_cols = cols.div_ceil(face_size);
    let mut blocks = hashbrown::HashSet::new();

    for bi in 0..block_rows {
        for bj in 0..block_cols {
            let mut has_tile = false;
            let row0 = bi * face_size;
            let col0 = bj * face_size;
            for i in row0..(row0 + face_size).min(rows) {
                let row = &board[i];
                for j in col0..(col0 + face_size).min(row.len()) {
                    if row[j].is_some() {
                        has_tile = true;
                        break;
                    }
                }
                if has_tile {
                    break;
                }
            }
            if has_tile {
                blocks.insert((bi, bj));
            }
        }
    }

    let start = blocks
        .iter()
        .copied()
        .min_by_key(|&(bi, bj)| (bi, bj))
        .unwrap();
    let mut orientations = hashbrown::HashMap::new();
    orientations.insert(
        start,
        Orientation {
            right: Vec3::new(1, 0, 0),
            down: Vec3::new(0, 1, 0),
            normal: Vec3::new(0, 0, 1),
        },
    );

    let mut queue = std::collections::VecDeque::new();
    queue.push_back(start);
    let directions = [(0isize, 1isize), (0, -1), (1, 0), (-1, 0)];

    while let Some((bi, bj)) = queue.pop_front() {
        let orientation = orientations[&(bi, bj)];
        for (di, dj) in directions {
            let nbi = bi as isize + di;
            let nbj = bj as isize + dj;
            if nbi < 0 || nbj < 0 {
                continue;
            }
            let nbi = nbi as usize;
            let nbj = nbj as usize;
            if !blocks.contains(&(nbi, nbj)) || orientations.contains_key(&(nbi, nbj)) {
                continue;
            }
            let move_dir = match (di, dj) {
                (0, 1) => MoveDir::Right,
                (0, -1) => MoveDir::Left,
                (1, 0) => MoveDir::Down,
                (-1, 0) => MoveDir::Up,
                _ => unreachable!(),
            };
            let next_orientation = orientation.rotate(move_dir);
            orientations.insert((nbi, nbj), next_orientation);
            queue.push_back((nbi, nbj));
        }
    }

    (face_size, orientations)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vec3 {
    const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }

    fn cross(self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

#[derive(Copy, Clone, Debug)]
enum MoveDir {
    Right,
    Left,
    Down,
    Up,
}

#[derive(Copy, Clone, Debug)]
struct Orientation {
    right: Vec3,
    down: Vec3,
    normal: Vec3,
}

impl Orientation {
    fn rotate(self, dir: MoveDir) -> Self {
        let (axis, sign) = match dir {
            MoveDir::Right => (self.down, 1),
            MoveDir::Left => (self.down, -1),
            MoveDir::Down => (self.right, -1),
            MoveDir::Up => (self.right, 1),
        };
        let rot = |v: Vec3| rotate_90(v, axis, sign);
        Self {
            right: rot(self.right),
            down: rot(self.down),
            normal: rot(self.normal),
        }
    }

    fn dir_from_vec(self, vec: Vec3) -> Dir {
        if vec == self.right {
            Dir::Right
        } else if vec == self.right.neg() {
            Dir::Left
        } else if vec == self.down {
            Dir::Down
        } else if vec == self.down.neg() {
            Dir::Up
        } else {
            panic!(
                "unknown direction vector: ({}, {}, {})",
                vec.x, vec.y, vec.z
            )
        }
    }

    fn axis_mapping(self, axis: Vec3) -> (Axis, bool) {
        if axis == self.down {
            (Axis::Row, false)
        } else if axis == self.down.neg() {
            (Axis::Row, true)
        } else if axis == self.right {
            (Axis::Col, false)
        } else if axis == self.right.neg() {
            (Axis::Col, true)
        } else {
            panic!("unknown edge axis: ({}, {}, {})", axis.x, axis.y, axis.z)
        }
    }
}

fn rotate_90(vec: Vec3, axis: Vec3, sign: i32) -> Vec3 {
    if vec == axis || vec == axis.neg() {
        return vec;
    }
    if sign == 1 {
        axis.cross(vec)
    } else {
        axis.neg().cross(vec)
    }
}

#[derive(Copy, Clone, Debug)]
struct EdgeTransition {
    to_face: Face,
    to_dir: Dir,
    k_axis: Axis,
    k_reversed: bool,
    fixed: usize,
}

fn rotation_for_edge(orientation: Orientation, dir: Dir) -> (Vec3, i32) {
    match dir {
        Dir::Right => (orientation.down, 1),
        Dir::Left => (orientation.down, -1),
        Dir::Down => (orientation.right, -1),
        Dir::Up => (orientation.right, 1),
    }
}

fn fixed_edge_coordinate(size: usize, dir: Dir, k_axis: Axis) -> usize {
    match (k_axis, dir) {
        (Axis::Row, Dir::Right) => 0,
        (Axis::Row, Dir::Left) => size - 1,
        (Axis::Col, Dir::Down) => 0,
        (Axis::Col, Dir::Up) => size - 1,
        _ => unreachable!("inconsistent cube edge mapping"),
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    const INPUT: &str = include_str!("../test_data/day_22.txt");

    #[test_case(EXAMPLE => 6032)]
    #[test_case(INPUT => 97356)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 5031)]
    #[test_case(INPUT => 120175)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
