use num::Complex;

pub fn part_1(input: &str) -> usize {
    let Complex { re, im } = parse_steps(input).sum();
    (re.unsigned_abs() + im.unsigned_abs()) / 2
}

pub fn part_2(input: &str) -> usize {
    parse_steps(input)
        .scan(Complex::new(0, 0), |pos, step| {
            *pos += step;
            Some(*pos)
        })
        .map(|Complex { re, im }| re.unsigned_abs() + im.unsigned_abs())
        .max()
        .unwrap()
        / 2
}

/// Double coordinate system
fn parse_steps(input: &str) -> impl Iterator<Item = Complex<isize>> {
    input.split(',').map(|dir| match dir {
        "n" => Complex::new(-2, 0),
        "ne" => Complex::new(-1, 1),
        "se" => Complex::new(1, 1),
        "s" => Complex::new(2, 0),
        "sw" => Complex::new(1, -1),
        "nw" => Complex::new(-1, -1),
        _ => panic!("invalid direction: {dir}"),
    })
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_11.txt");

    #[test_case("ne,ne,ne" => 3)]
    #[test_case("ne,ne,sw,sw" => 0)]
    #[test_case("ne,ne,s,s" => 2)]
    #[test_case("se,sw,se,sw,sw" => 3)]
    #[test_case(INPUT => 784)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(INPUT => 1558)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
