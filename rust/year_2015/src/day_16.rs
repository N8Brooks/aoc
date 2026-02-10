pub fn part_1(input: &str) -> usize {
    parse_compounds(input)
        .position(|compounds| {
            matches!(
                compounds,
                Compounds {
                    children: Some(3) | None,
                    cats: Some(7) | None,
                    samoyeds: Some(2) | None,
                    pomeranians: Some(3) | None,
                    akitas: Some(0) | None,
                    vizslas: Some(0) | None,
                    goldfish: Some(5) | None,
                    trees: Some(3) | None,
                    cars: Some(2) | None,
                    perfumes: Some(1) | None,
                }
            )
        })
        .unwrap()
        + 1
}

pub fn part_2(input: &str) -> usize {
    parse_compounds(input)
        .position(|compounds| {
            matches!(
                compounds,
                Compounds {
                    children: Some(3) | None,
                    cats: Some(8..) | None,
                    samoyeds: Some(2) | None,
                    pomeranians: Some(..3) | None,
                    akitas: Some(0) | None,
                    vizslas: Some(0) | None,
                    goldfish: Some(..5) | None,
                    trees: Some(4..) | None,
                    cars: Some(2) | None,
                    perfumes: Some(1) | None,
                }
            )
        })
        .unwrap()
        + 1
}

fn parse_compounds(input: &str) -> impl Iterator<Item = Compounds> {
    input.lines().map(|line| {
        let mut compounds = Compounds::default();
        let (_sue, parts) = line.split_once(": ").unwrap();
        for part in parts.split(", ") {
            let (name, value) = part.split_once(": ").unwrap();
            let entry = match name {
                "children" => &mut compounds.children,
                "cats" => &mut compounds.cats,
                "samoyeds" => &mut compounds.samoyeds,
                "pomeranians" => &mut compounds.pomeranians,
                "akitas" => &mut compounds.akitas,
                "vizslas" => &mut compounds.vizslas,
                "goldfish" => &mut compounds.goldfish,
                "trees" => &mut compounds.trees,
                "cars" => &mut compounds.cars,
                "perfumes" => &mut compounds.perfumes,
                _ => panic!("unknown compound {name}"),
            };
            *entry = Some(value.parse().unwrap());
        }
        compounds
    })
}

#[derive(Default)]
struct Compounds {
    children: Option<usize>,
    cats: Option<usize>,
    samoyeds: Option<usize>,
    pomeranians: Option<usize>,
    akitas: Option<usize>,
    vizslas: Option<usize>,
    goldfish: Option<usize>,
    trees: Option<usize>,
    cars: Option<usize>,
    perfumes: Option<usize>,
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_16.txt");

    #[test_case(INPUT => 213)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(INPUT => 323)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
