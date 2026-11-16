pub fn part_1(input: &str) -> usize {
    let (min, buses) = input.split_once('\n').unwrap();
    let min: usize = min.parse().unwrap();

    let (bus, wait) = buses
        .split(',')
        .filter(|&s| s != "x")
        .map(|s| s.parse().unwrap())
        .map(|bus| (bus, min.next_multiple_of(bus) - min))
        .min_by_key(|&(_, wait)| wait)
        .unwrap();

    bus * wait
}

pub fn part_2(input: &str) -> usize {
    let (_, buses) = input.split_once('\n').unwrap();

    let (time, _) = buses
        .split(',')
        .enumerate()
        .filter(|&(_, bus)| bus != "x")
        .map(|(offset, bus)| (offset, bus.parse::<usize>().unwrap()))
        .fold((0, 1), |(time, step), (offset, bus)| {
            let time = (time..)
                .step_by(step)
                .find(|&time| (time + offset).is_multiple_of(bus))
                .unwrap();
            (time, step * bus)
        });

    time
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_13.txt");

    const EXAMPLE: &str = "939
7,13,x,x,59,x,31,19";

    #[test_case(EXAMPLE => 295)]
    #[test_case(INPUT => 4722)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 1068781)]
    #[test_case(INPUT => 825305207525452)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
