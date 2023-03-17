use ahash::AHashSet;
use itertools::Itertools;
use num::Complex;

pub fn part_1(input: &str) -> usize {
    min_steps_to_checkpoint(input, 1)
}

pub fn part_2(input: &str) -> usize {
    min_steps_to_checkpoint(input, 3)
}

const MOVES: [Complex<isize>; 5] = [
    Complex::new(0, 1),
    Complex::new(-1, 0),
    Complex::new(0, -1),
    Complex::new(1, 0),
    Complex::new(0, 0),
];

pub fn min_steps_to_checkpoint(input: &str, checkpoint_index: usize) -> usize {
    let (mut blizzard_locations, blizzard_directions) = parse_blizzards(input);
    let max_i = input.lines().count() as isize - 2;
    let max_j = input.lines().next().unwrap().len() as isize - 2;
    let start = Complex::new(-1, 0);
    let goal = Complex::new(max_i, max_j - 1);
    let mut step = 0;
    let mut locations = vec![(0, start)];
    while !locations.contains(&(checkpoint_index, goal)) {
        let blizzards: AHashSet<_> = blizzard_locations
            .iter_mut()
            .zip(&blizzard_directions)
            .map(|(location, direction)| {
                let Complex { re: i, im: j } = *location + direction;
                *location = Complex::new(i.rem_euclid(max_i), j.rem_euclid(max_j));
                location
            })
            .collect();
        locations = locations
            .into_iter()
            .flat_map(|(checkpoint, location)| {
                MOVES
                    .iter()
                    .map(move |direction| (checkpoint, location + direction))
            })
            .filter_map(|(checkpoint, location @ Complex { re: i, im: j })| {
                if (0..max_i).contains(&i) && (0..max_j).contains(&j) {
                    Some((checkpoint, location))
                } else if location == start {
                    Some((checkpoint + checkpoint % 2, location))
                } else if location == goal {
                    Some((checkpoint + 1 - checkpoint % 2, location))
                } else {
                    None
                }
            })
            .filter(|(_, location)| !blizzards.contains(location))
            .unique()
            .max_set_by_key(|(checkpoint, _)| *checkpoint); // Checkpoints are 'safe'
        step += 1;
    }
    step
}

fn parse_blizzards(input: &str) -> (Vec<Complex<isize>>, Vec<Complex<isize>>) {
    input
        .lines()
        .skip(1)
        .enumerate()
        .flat_map(|(i, line)| {
            let i = isize::try_from(i).unwrap();
            line.bytes()
                .skip(1)
                .enumerate()
                .filter_map(move |(j, byte)| {
                    let direction = match byte {
                        b'>' => Complex::new(0, 1),
                        b'^' => Complex::new(-1, 0),
                        b'<' => Complex::new(0, -1),
                        b'v' => Complex::new(1, 0),
                        _ => return None,
                    };
                    let location = Complex::new(i, isize::try_from(j).unwrap());
                    Some((location, direction))
                })
        })
        .unzip()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    const INPUT: &str = include_str!("../../../testdata/year_2022/day_24.txt");

    #[test_case(EXAMPLE, 18)]
    #[test_case(INPUT, 279)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 54)]
    #[test_case(INPUT, 762)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
