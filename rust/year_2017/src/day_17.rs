use std::iter;

pub fn part_1(input: &str) -> usize {
    const N: usize = 2017;
    let step: usize = input.trim_end().parse().unwrap();
    let (buffer, position) = (1..N).fold(([0; N], 1), |(mut buffer, position), value| {
        buffer[position..=value].shift_right([value]);
        (buffer, (position + step) % (value + 1) + 1)
    });
    buffer[position % N]
}

pub fn part_2(input: &str) -> usize {
    const N: usize = 50_000_000;
    let step: usize = input.trim().parse().unwrap();

    if step == 0 {
        return 1;
    }

    let mut values = 1..=N;
    let mut position = 0;
    iter::from_fn(|| {
        loop {
            let value = values.next()?;
            position = (position + step) % value + 1;

            if position == 1 {
                return Some(value);
            }

            let skip = ((value - position) / step).min(N - value);
            if skip > 0 {
                position += skip * (step + 1);
                values.nth(skip - 1);
            }
        }
    })
    .last()
    .unwrap()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_17.txt");

    #[test_case("3" => 638)]
    #[test_case(INPUT => 866)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(INPUT => 11995607)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
