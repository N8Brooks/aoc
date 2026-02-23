use hashbrown::HashSet;

pub fn part_1(input: &str) -> u32 {
    let (mut i, mut j) = (0, 0);
    for ((di, dj), steps) in parse_input(input) {
        i += di * steps;
        j += dj * steps;
    }
    i.unsigned_abs() + j.unsigned_abs()
}

pub fn part_2(input: &str) -> u32 {
    let (mut i, mut j) = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::from([(i, j)]);
    for ((di, dj), steps) in parse_input(input) {
        for _ in 0..steps {
            i += di;
            j += dj;
            if !visited.insert((i, j)) {
                return i.unsigned_abs() + j.unsigned_abs();
            }
        }
    }
    unreachable!("no location visited twice")
}

fn parse_input(input: &str) -> impl Iterator<Item = ((i32, i32), i32)> {
    let (mut di, mut dj) = (-1, 0);
    input.split(", ").map(move |s| {
        let (turn, steps) = s.split_at(1);
        let steps = steps.parse().unwrap();
        (di, dj) = match turn {
            "R" => (dj, -di),
            "L" => (-dj, di),
            _ => panic!("invalid turn: {s}"),
        };
        ((di, dj), steps)
    })
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_01.txt");

    #[test_case("R2, L3" => 5)]
    #[test_case("R2, R2, R2" => 2)]
    #[test_case("R5, L5, R5, R3" => 12)]
    #[test_case(INPUT => 161)]
    fn part_1(input: &str) -> u32 {
        super::part_1(input)
    }

    #[test_case("R8, R4, R4, R8" => 4)]
    #[test_case(INPUT => 110)]
    fn part_2(input: &str) -> u32 {
        super::part_2(input)
    }
}
