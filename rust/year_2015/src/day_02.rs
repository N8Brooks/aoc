use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    parse_input(input)
        .map(|[l, w, h]| {
            let side_areas = [l * w, w * h, h * l];
            let surface_area: usize = side_areas.iter().map(|area| 2 * area).sum();
            let min_side_area = side_areas.into_iter().min().unwrap();
            surface_area + min_side_area
        })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    parse_input(input)
        .map(|[l, w, h]| {
            let perimeters = [2 * (l + w), 2 * (w + h), 2 * (h + l)];
            let min_perimeter = perimeters.into_iter().min().unwrap();
            let volume = l * w * h;
            min_perimeter + volume
        })
        .sum()
}

fn parse_input(input: &str) -> impl Iterator<Item = [usize; 3]> {
    input.lines().map(|line| {
        line.split('x')
            .map(|n| n.parse().unwrap())
            .collect_array()
            .unwrap()
    })
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_02.txt");

    const EXAMPLE_1: &str = "2x3x4";

    const EXAMPLE_2: &str = "1x1x10";

    #[test_case(EXAMPLE_1 => 58)]
    #[test_case(EXAMPLE_2 => 43)]
    #[test_case(INPUT => 1606483)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE_1 => 34)]
    #[test_case(EXAMPLE_2 => 14)]
    #[test_case(INPUT => 3842356)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
