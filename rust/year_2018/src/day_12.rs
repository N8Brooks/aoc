use hashbrown::HashMap;
use itertools::{repeat_n, Itertools};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct IsPlant(bool);

impl From<u8> for IsPlant {
    fn from(byte: u8) -> Self {
        match byte {
            b'.' => IsPlant(false),
            b'#' => IsPlant(true),
            _ => panic!("unrecognized plant byte {byte}"),
        }
    }
}

type NoteInput = (IsPlant, IsPlant, IsPlant, IsPlant, IsPlant);

pub fn part_1(input: &str) -> isize {
    simulate_plants(input)
        .nth(20)
        .unwrap_or_else(|| unreachable!())
}

pub fn part_2(input: &str) -> isize {
    const SKIP_GENERATIONS: usize = 1000;
    let (x, d_x) = simulate_plants(input)
        .tuple_windows()
        .map(|(a, b)| (a, b - a))
        .nth(SKIP_GENERATIONS)
        .unwrap_or_else(|| unreachable!());
    x + (50_000_000_000 - SKIP_GENERATIONS) as isize * d_x
}

fn simulate_plants(input: &str) -> impl Iterator<Item = isize> {
    let (offset, state, notes) = parse_input(input);
    Some(sum_plant_indexes(&offset, &state))
        .into_iter()
        .chain((1..).scan((offset, state), move |(offset, state), _| {
            *offset -= 2;
            *state = repeat_n(IsPlant(false), 4)
                .chain(state.iter().copied())
                .chain(repeat_n(IsPlant(false), 4))
                .tuple_windows()
                .map(|note_input: NoteInput| {
                    notes.get(&note_input).copied().unwrap_or(IsPlant(false))
                })
                .skip_while(|is_plant| {
                    if is_plant.0 {
                        false
                    } else {
                        *offset += 1;
                        true
                    }
                })
                .collect();
            while state.last().is_some_and(|is_plant| !is_plant.0) {
                state.pop();
            }
            Some(sum_plant_indexes(offset, state))
        }))
}

fn sum_plant_indexes(offset: &isize, state: &Vec<IsPlant>) -> isize {
    (*offset..)
        .zip(state)
        .filter(|(_, is_plant)| is_plant.0)
        .map(|(i, _)| i)
        .sum()
}

fn parse_input(input: &str) -> (isize, Vec<IsPlant>, HashMap<NoteInput, IsPlant>) {
    let (initial_state, notes) = input.split_once("\n\n").unwrap();
    let state = initial_state
        .strip_prefix("initial state: ")
        .unwrap()
        .bytes()
        .map(IsPlant::from)
        .collect_vec();
    let notes = notes
        .lines()
        .map(|line| {
            let (input, output) = line.split_once(" => ").unwrap();
            let input: NoteInput = input.bytes().map(IsPlant::from).collect_tuple().unwrap();
            let output = output.bytes().map(IsPlant::from).exactly_one().unwrap();
            (input, output)
        })
        .collect::<HashMap<_, _>>();
    (0, state, notes)
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";

    const INPUT: &str = include_str!("../../../test_data/year_2018/day_12.txt");

    #[test_case(EXAMPLE, 325; "example")]
    #[test_case(INPUT, 3915; "input")]
    fn part_1(input: &str, expected: isize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(INPUT, 4900000001793; "input")]
    fn part_2(input: &str, expected: isize) {
        assert_eq!(super::part_2(input), expected);
    }
}
