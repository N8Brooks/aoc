use hashbrown::HashSet;

pub fn part_1(input: &str) -> usize {
    let (molecule, replacements) = parse_input(input);
    replacements
        .flat_map(|(from, to)| {
            molecule.match_indices(from).map(move |(i, _)| {
                let mut molecule = molecule.to_string();
                molecule.replace_range(i..i + from.len(), to);
                molecule
            })
        })
        .collect::<HashSet<_>>()
        .len()
}

pub fn part_2(input: &str) -> usize {
    let (molecule, _) = parse_input(input);
    let n = molecule.matches(char::is_uppercase).count();
    let rn = molecule.matches("Rn").count();
    let ar = molecule.matches("Ar").count();
    let y = molecule.matches("Y").count();
    n - rn - ar - 2 * y - 1
}

fn parse_input(input: &str) -> (&str, impl Iterator<Item = (&str, &str)>) {
    let (replacements, molecule) = input.rsplit_once("\n\n").unwrap();
    let replacements = replacements
        .lines()
        .map(|line| line.split_once(" => ").unwrap());
    (molecule, replacements)
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_19.txt");

    const EXAMPLE_1: &str = "H => HO
H => OH
O => HH

HOH";

    const EXAMPLE_2: &str = "H => HO
H => OH
O => HH

HOHOHO";

    #[test_case(EXAMPLE_1 => 4)]
    #[test_case(EXAMPLE_2 => 7)]
    #[test_case(INPUT => 576)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    //     const EXAMPLE_3: &str = "e => H
    // e => O
    // H => HO
    // H => OH
    // O => HH
    //
    // HOH";

    //     const EXAMPLE_4: &str = "e => H
    // e => O
    // H => HO
    // H => OH
    // O => HH
    //
    // HOHOHO";

    // #[test_case(EXAMPLE_3 => 3)]
    // #[test_case(EXAMPLE_4 => 6)]
    #[test_case(INPUT => 207)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
