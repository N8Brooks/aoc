use itertools::Itertools as _;

pub fn part_1(input: &str) -> usize {
    let (shape_areas, regions) = parse_input(input);

    let lo = regions
        .iter()
        .filter(|(area, counts)| {
            let capacity = area / 9;
            let required = counts.iter().sum();
            capacity >= required
        })
        .count();

    let hi = regions
        .iter()
        .filter(|(capacity, shape_counts)| {
            let required = shape_counts
                .iter()
                .zip(shape_areas)
                .map(|(count, area)| count * area)
                .sum();
            *capacity >= required
        })
        .count();

    lo.midpoint(hi)
}

fn parse_input(input: &str) -> ([usize; 6], Vec<(usize, [usize; 6])>) {
    let (shapes, regions) = input.rsplit_once("\n\n").unwrap();
    let shape_areas = shapes
        .split("\n\n")
        .map(|shape| {
            shape
                .lines()
                .skip(1) // index line
                .flat_map(|line| line.bytes())
                .filter(|&c| c == b'#')
                .count()
        })
        .collect_array()
        .unwrap();
    let regions = regions
        .lines()
        .map(|line| {
            let (dims, counts) = line.split_once(": ").unwrap();
            let (w, h) = dims.split_once('x').unwrap();
            let w: usize = w.parse().unwrap();
            let h: usize = h.parse().unwrap();
            let counts = counts
                .split(' ')
                .map(|num| num.parse().unwrap())
                .collect_array()
                .unwrap();
            (w * h, counts)
        })
        .collect();
    (shape_areas, regions)
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_12.txt");

    const EXAMPLE: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[test_case(EXAMPLE => 2)]
    #[test_case(INPUT => 521)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }
}
