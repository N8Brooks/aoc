pub fn part_1(input: &str) -> usize {
    let mut lines = input.lines();
    let line0 = lines.next().unwrap().as_bytes();
    let m = line0.len();
    let start = line0.iter().position(|&b| b == b'S').unwrap();
    let mut beams = vec![false; m];
    beams[start] = true;
    let mut count = 0;
    for line in lines {
        for (i, b) in line.bytes().enumerate() {
            if b == b'^' && beams[i] {
                beams[i - 1] = true;
                beams[i] = false;
                beams[i + 1] = true;
                count += 1;
            }
        }
    }
    count
}

pub fn part_2(input: &str) -> usize {
    let mut lines = input.lines();
    let line0 = lines.next().unwrap().as_bytes();
    let m = line0.len();
    let start = line0.iter().position(|&b| b == b'S').unwrap();
    let mut timelines = vec![0; m];
    timelines[start] = 1;
    for line in lines {
        for (i, b) in line.bytes().enumerate() {
            if b == b'^' && timelines[i] > 0 {
                timelines[i - 1] += timelines[i];
                timelines[i + 1] += timelines[i];
                timelines[i] = 0;
            }
        }
    }
    timelines.into_iter().sum()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_07.txt");

    const EXAMPLE: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test_case(EXAMPLE => 21)]
    #[test_case(INPUT => 1667)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 40)]
    #[test_case(INPUT => 62943905501815)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
