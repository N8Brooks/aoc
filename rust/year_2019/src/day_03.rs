use num::Complex;

pub fn part_1(input: &str) -> usize {
    use hashbrown::HashSet;
    let (wire_1, wire_2) = input.split_once('\n').unwrap();
    let set_1: HashSet<_> = iter_wire(wire_1).collect();
    iter_wire(wire_2)
        .filter(|pos| set_1.contains(pos))
        .map(|pos| pos.re.unsigned_abs() + pos.im.unsigned_abs())
        .min()
        .unwrap()
}

pub fn part_2(input: &str) -> usize {
    use hashbrown::HashMap;
    let (wire_1, wire_2) = input.split_once('\n').unwrap();
    let map_1 = iter_wire(wire_1)
        .enumerate()
        .fold(HashMap::new(), |mut map_1, (i, pos)| {
            map_1.entry(pos).or_insert(i + 1);
            map_1
        });
    iter_wire(wire_2)
        .enumerate()
        .filter_map(|(j, pos)| map_1.get(&pos).map(|i| i + j + 1))
        .min()
        .unwrap()
}

fn iter_wire(wire: &str) -> impl Iterator<Item = Complex<isize>> {
    wire.split(',')
        .flat_map(|segment| {
            use std::iter::repeat_n;
            let (dir, mag) = segment.split_at(1);
            let dir = match dir {
                "R" => Complex::new(1, 0),
                "L" => Complex::new(-1, 0),
                "U" => Complex::new(0, 1),
                "D" => Complex::new(0, -1),
                _ => panic!("invalid direction"),
            };
            let mag = mag.parse().unwrap();
            repeat_n(dir, mag)
        })
        .map({
            let mut pos = Complex::new(0, 0);
            move |d| {
                pos += d;
                pos
            }
        })
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE_1: &str = "R8,U5,L5,D3
U7,R6,D4,L4";

    const EXAMPLE_2: &str = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";

    const EXAMPLE_3: &str = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

    const INPUT: &str = include_str!("../test_data/day_03.txt");

    #[test_case(EXAMPLE_1, 6)]
    #[test_case(EXAMPLE_2, 159)]
    #[test_case(EXAMPLE_3, 135)]
    #[test_case(INPUT, 1674)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE_1, 30)]
    #[test_case(EXAMPLE_2, 610)]
    #[test_case(EXAMPLE_3, 410)]
    #[test_case(INPUT, 14012)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
