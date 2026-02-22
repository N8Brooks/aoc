pub fn part_1(input: &str) -> usize {
    let target: u32 = input.trim().parse().unwrap();
    let max_house = (target / 10 + 1).try_into().unwrap();
    let mut presents = vec![0; max_house + 1];

    for elf in 1..=max_house {
        let gift = 10 * elf as u32;
        presents.iter_mut().step_by(elf).skip(1).for_each(|p| {
            *p += gift;
        })
    }

    presents
        .into_iter()
        .enumerate()
        .skip(1)
        .find(|&(_, p)| p >= target)
        .unwrap()
        .0
}

pub fn part_2(input: &str) -> usize {
    let target: u32 = input.trim().parse().unwrap();
    let max_house = target.div_ceil(11).try_into().unwrap();
    let mut presents = vec![0; max_house + 1];

    for elf in 1..=max_house {
        let gift = 11 * elf as u32;
        presents
            .iter_mut()
            .step_by(elf)
            .skip(1)
            .take(50)
            .for_each(|p| {
                *p += gift;
            });
    }

    presents
        .into_iter()
        .enumerate()
        .skip(1)
        .find(|&(_, p)| p >= target)
        .unwrap()
        .0
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_20.txt");

    #[test_case(INPUT => 776160)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(INPUT => 786240)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
