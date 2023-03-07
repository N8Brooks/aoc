use std::iter::repeat;

use num::Integer;

#[derive(Debug)]
enum Shape {
    Horizontal = 0x2500,
    VerticalAndHorizontal = 0x253C,
    UpAndLeft = 0x2518,
    Vertical = 0x2502,
    Block = 0x2580,
}

impl Shape {
    fn from_index(shape_index: &usize) -> Shape {
        match shape_index {
            0 => Shape::Horizontal,
            1 => Shape::VerticalAndHorizontal,
            2 => Shape::UpAndLeft,
            3 => Shape::Vertical,
            4 => Shape::Block,
            _ => panic!("expected remainder in `0..5`"),
        }
    }

    fn get_coordinate_translations(&self) -> &'static [(usize, usize)] {
        match self {
            Shape::Horizontal => &[(0, 0), (0, 1), (0, 2), (0, 3)],
            Shape::VerticalAndHorizontal => &[(2, 1), (1, 0), (1, 1), (1, 2), (0, 1)],
            Shape::UpAndLeft => &[(2, 2), (1, 2), (0, 0), (0, 1), (0, 2)],
            Shape::Vertical => &[(3, 0), (2, 0), (1, 0), (0, 0)],
            Shape::Block => &[(1, 0), (1, 1), (0, 0), (0, 1)],
        }
    }

    fn get_coordinates(&self, (i, j): (usize, usize)) -> Vec<(usize, usize)> {
        self.get_coordinate_translations()
            .iter()
            .map(|(i_d, j_d)| (i + i_d, j + j_d))
            .collect()
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Left = -1,
    Right = 1,
}

impl TryFrom<u8> for Direction {
    type Error = ();

    fn try_from(byte: u8) -> Result<Direction, ()> {
        match byte {
            b'<' => Ok(Direction::Left),
            b'>' => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

impl Direction {
    fn as_col_delta(self) -> isize {
        self as isize
    }
}

#[derive(Debug, Default)]
struct Simulation {
    chamber: Vec<[bool; 7]>,
}

impl Simulation {
    fn get_spawn_coordinates(&self) -> (usize, usize) {
        (self.chamber.len() + 3, 2)
    }

    fn move_horizontally(
        &self,
        coordinates: &[(usize, usize)],
        direction: Direction,
    ) -> Result<Vec<(usize, usize)>, ()> {
        let col_delta = direction.as_col_delta();
        coordinates
            .iter()
            .map(|(i, j)| match j.checked_add_signed(col_delta) {
                Some(j @ 0..=6) if !self.get_default(*i, j) => Ok((*i, j)),
                _ => Err(()),
            })
            .collect::<Result<_, _>>()
    }

    fn move_vertically(&self, coordinates: &[(usize, usize)]) -> Result<Vec<(usize, usize)>, ()> {
        coordinates
            .iter()
            .map(|(i, j)| match i.checked_sub(1) {
                Some(i) if !self.get_default(i, *j) => Ok((i, *j)),
                _ => Err(()),
            })
            .collect::<Result<_, _>>()
    }

    fn get_default(&self, i: usize, j: usize) -> bool {
        self.chamber.get(i).map(|row| row[j]).unwrap_or(false)
    }

    fn insert_coordinates(&mut self, coordinates: &[(usize, usize)]) {
        let max_i = coordinates.iter().map(|(i, _)| i).max().unwrap();
        let delta_i = (max_i + 1).saturating_sub(self.chamber.len());
        self.chamber.extend(repeat([false; 7]).take(delta_i));
        coordinates
            .iter()
            .for_each(|(i, j)| self.chamber[*i][*j] = true);
    }

    fn get_max_i(&self) -> usize {
        self.chamber
            .iter()
            .rposition(|row| row.iter().any(|taken| *taken))
            .unwrap_or(0)
    }

    fn simulate(&mut self, input: &str, n_turns: usize) -> usize {
        let n_directions = input.len();
        let mut directions = input
            .bytes()
            .map(|byte| Direction::try_from(byte).unwrap())
            .enumerate()
            .cycle()
            .peekable();

        // Caching inspired by /u/4HbQ :)
        let mut memo = vec![[None; 5]; input.len()];

        for turn_index in 0..n_turns {
            let direction_index = directions.peek().unwrap().0 % n_directions;
            let shape_index = turn_index % 5;

            if let Some((pre_turn_index, pre_max_i)) = memo[direction_index][shape_index] {
                let (quotient, remainder) =
                    (n_turns - turn_index).div_mod_floor(&(turn_index - pre_turn_index));
                if remainder == 0 {
                    let cur_max_i = self.get_max_i();
                    return cur_max_i + (cur_max_i - pre_max_i) * quotient + 1;
                }
            } else {
                memo[direction_index][shape_index] = Some((turn_index, self.get_max_i()))
            }

            let shape = Shape::from_index(&shape_index);
            let coordinates = self.get_spawn_coordinates();
            let mut coordinates = shape.get_coordinates(coordinates);
            while {
                let (_, direction) = directions.next().unwrap();
                if let Ok(new_coordinates) = self.move_horizontally(&coordinates, direction) {
                    coordinates = new_coordinates;
                }
                if let Ok(new_coordinates) = self.move_vertically(&coordinates) {
                    coordinates = new_coordinates;
                    true
                } else {
                    self.insert_coordinates(&coordinates);

                    false
                }
            } {}
        }

        self.get_max_i() + 1
    }
}

pub fn part_1(input: &str) -> usize {
    let mut chamber = Simulation::default();
    chamber.simulate(input, 2022)
}

pub fn part_2(input: &str) -> usize {
    let mut chamber = Simulation::default();
    chamber.simulate(input, 1000000000000)
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    const INPUT: &str = include_str!("../../../testdata/year_2022/day_17.txt");

    #[test_case(EXAMPLE, 3068)]
    #[test_case(INPUT, 3098)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 1514285714288)]
    #[test_case(INPUT, 1525364431487)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
