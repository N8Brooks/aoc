use num::Integer as _;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Unknown,
    Wall,
    Empty,
    Oxygen,
}

const START: (usize, usize) = (21, 21);

pub fn part_1(input: &str) -> usize {
    let program = parse_program(input);
    let mut map = parse_maze(program);

    let mut stack = vec![(START.0, START.1, 0)];
    while let Some((i, j, dist)) = stack.pop() {
        match map[i][j] {
            Tile::Unknown | Tile::Wall => {}
            Tile::Empty => {
                map[i][j] = Tile::Unknown;
                stack.extend([
                    (i + 1, j, dist + 1),
                    (i - 1, j, dist + 1),
                    (i, j + 1, dist + 1),
                    (i, j - 1, dist + 1),
                ]);
            }
            Tile::Oxygen => {
                return dist as usize;
            }
        }
    }
    0
}

pub fn part_2(input: &str) -> usize {
    let program = parse_program(input);
    let mut map = parse_maze(program);
    let (oi, oj) = map
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find_map(|(j, &tile)| (tile == Tile::Oxygen).then_some((i, j)))
        })
        .unwrap();
    let mut stack_1 = vec![(oi, oj)];
    let mut stack_2 = Vec::new();
    let mut minutes = 0;
    while !stack_1.is_empty() {
        for (i1, j2) in stack_1.drain(..) {
            for (i1, i2) in [(i1 + 1, j2), (i1 - 1, j2), (i1, j2 + 1), (i1, j2 - 1)] {
                if map[i1][i2] == Tile::Empty {
                    map[i1][i2] = Tile::Oxygen;
                    stack_2.push((i1, i2));
                }
            }
        }
        std::mem::swap(&mut stack_1, &mut stack_2);
        minutes += 1;
    }
    minutes - 1
}

fn parse_maze(program: Vec<isize>) -> [[Tile; 43]; 43] {
    let mut intcode = Intcode::new(program);
    const DIRS: [isize; 4] = [1, 4, 2, 3]; // north, south, west, east
    let right = |dir: isize| {
        let i = DIRS.iter().position(|&d| d == dir).unwrap();
        DIRS[(i + 1) % 4]
    };
    let left = |dir: isize| {
        let i = DIRS.iter().position(|&d| d == dir).unwrap();
        DIRS[(i + 3) % 4]
    };

    let mut input = 1;
    let (mut i1, mut j1) = START;
    let mut map = [[Tile::Unknown; 43]; 43];

    while {
        let output = intcode.next(input);
        let (i2, j2) = match input {
            1 => (i1, j1 + 1),
            2 => (i1, j1 - 1),
            3 => (i1 - 1, j1),
            4 => (i1 + 1, j1),
            _ => panic!("unexpected input {input}"),
        };
        match output {
            0 => {
                map[i2][j2] = Tile::Wall;
                input = left(input);
            }
            1 => {
                map[i2][j2] = Tile::Empty;
                (i1, j1) = (i2, j2);
                input = right(input);
            }
            2 => {
                map[i2][j2] = Tile::Oxygen;
                (i1, j1) = (i2, j2);
                input = right(input);
            }
            _ => panic!("unexpected output {output}"),
        }
        input != 1 || (i1, j1) != (21, 21)
    } {}

    map
}

struct Intcode {
    /// Computer's memory
    memory: Vec<isize>,
    /// Instruction pointer
    ip: usize,
    /// Relative base
    rb: isize,
}

fn parse_program(input: &str) -> Vec<isize> {
    input.split(',').map(|num| num.parse().unwrap()).collect()
}

impl Intcode {
    fn new(memory: Vec<isize>) -> Self {
        Self {
            memory,
            ip: 0,
            rb: 0,
        }
    }

    fn next(&mut self, input: isize) -> isize {
        let mut input = Some(input);
        loop {
            let instruction = self.fetch();
            let (modes, opcode) = instruction.div_rem(&100);
            let (modes, mode_1) = modes.div_rem(&10);
            let (mode_3, mode_2) = modes.div_rem(&10);
            match opcode {
                1 => {
                    let param_1 = self.read(mode_1);
                    let param_2 = self.read(mode_2);
                    self.write(param_1 + param_2, mode_3);
                }
                2 => {
                    let param_1 = self.read(mode_1);
                    let param_2 = self.read(mode_2);
                    self.write(param_1 * param_2, mode_3);
                }
                3 => {
                    let input = input.take().unwrap();
                    self.write(input, mode_1);
                }
                4 => return self.read(mode_1),
                5 => {
                    let param_1 = self.read(mode_1);
                    let param_2 = self.read(mode_2);
                    if param_1 != 0 {
                        self.ip = param_2.try_into().unwrap();
                    }
                }
                6 => {
                    let param_1 = self.read(mode_1);
                    let param_2 = self.read(mode_2);
                    if param_1 == 0 {
                        self.ip = param_2.try_into().unwrap();
                    }
                }
                7 => {
                    let param_1 = self.read(mode_1);
                    let param_2 = self.read(mode_2);
                    self.write((param_1 < param_2).into(), mode_3);
                }
                8 => {
                    let param_1 = self.read(mode_1);
                    let param_2 = self.read(mode_2);
                    self.write((param_1 == param_2).into(), mode_3);
                }
                9 => {
                    let param_1 = self.read(mode_1);
                    self.rb += param_1;
                }
                _ => panic!("unexpected opcode {opcode}"),
            }
        }
    }

    fn read(&mut self, mode: isize) -> isize {
        let word = self.fetch();
        let i = match mode {
            0 => word,
            1 => return word,
            2 => self.rb + word,
            _ => panic!("unexpected mode {mode}"),
        };
        let u: usize = i.try_into().unwrap();
        self.memory.get(u).copied().unwrap_or(0)
    }

    fn write(&mut self, value: isize, mode: isize) {
        let word = self.fetch();
        let i = match mode {
            0 => word,
            2 => self.rb + word,
            _ => panic!("unexpected mode {mode}"),
        };
        let u: usize = i.try_into().unwrap();
        if u >= self.memory.len() {
            self.memory.resize(u + 1, 0);
        }
        self.memory[u] = value;
    }

    fn fetch(&mut self) -> isize {
        let word = self.memory[self.ip];
        self.ip += 1;
        word
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_15.txt");

    #[test_case(INPUT, 266)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(INPUT, 274)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
