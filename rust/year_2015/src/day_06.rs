pub fn part_1(input: &str) -> usize {
    let mut lights = vec![[false; 1000]; 1000];
    for (action, (i0, i1), (j0, j1)) in parse_input(input) {
        match action {
            TurnOn | TurnOff => lights[i0..=i1]
                .iter_mut()
                .for_each(|row| row[j0..=j1].fill(action == TurnOn)),
            Toggle => lights[i0..=i1]
                .iter_mut()
                .flat_map(|row| &mut row[j0..=j1])
                .for_each(|light| *light = !*light),
        }
    }
    lights.into_iter().flatten().filter(|&light| light).count()
}

pub fn part_2(input: &str) -> usize {
    let mut lights = vec![[0u8; 1000]; 1000];
    for (action, (i0, i1), (j0, j1)) in parse_input(input) {
        let action = action as i8;
        lights[i0..=i1]
            .iter_mut()
            .flat_map(|row| &mut row[j0..=j1])
            .for_each(|light| *light = light.saturating_add_signed(action));
    }
    lights
        .into_iter()
        .flatten()
        .map(|light| light as usize)
        .sum()
}

#[derive(PartialEq, Eq)]
enum Action {
    TurnOn = 1,
    TurnOff = -1,
    Toggle = 2,
}

use Action::*;

fn parse_input(input: &str) -> impl Iterator<Item = (Action, (usize, usize), (usize, usize))> {
    input.lines().map(|line| {
        let (action, ranges) = if let Some(ranges) = line.strip_prefix("turn on ") {
            (TurnOn, ranges)
        } else if let Some(ranges) = line.strip_prefix("turn off ") {
            (TurnOff, ranges)
        } else if let Some(ranges) = line.strip_prefix("toggle ") {
            (Toggle, ranges)
        } else {
            panic!("invalid action: {line}");
        };
        let (start, end) = ranges.split_once(" through ").unwrap();
        let (i0, j0) = start.split_once(',').unwrap();
        let (i0, j0) = (i0.parse().unwrap(), j0.parse().unwrap());
        let (i1, j1) = end.split_once(',').unwrap();
        let (i1, j1) = (i1.parse().unwrap(), j1.parse().unwrap());
        (action, (i0, i1), (j0, j1))
    })
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_06.txt");

    #[test_case(INPUT => 543903)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(INPUT => 14687245)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
