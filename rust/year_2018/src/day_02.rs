use itertools::Itertools;

pub fn part_1(input: &str) -> usize {
    let counts = input
        .lines()
        .flat_map(|line| line.chars().counts().into_values().unique())
        .counts();
    [2, 3]
        .iter()
        .map(|count| counts.get(count).unwrap_or(&0))
        .product()
}

pub fn part_2(input: &str) -> String {
    input
        .lines()
        .flat_map(|line| {
            (0..line.len()).map(|i| {
                line.chars()
                    .enumerate()
                    .map(|(j, char)| (i != j).then_some(char))
                    .collect_vec()
            })
        })
        .duplicates()
        .next()
        .expect("some duplicate")
        .into_iter()
        .flatten()
        .collect::<String>()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE_1: &str = "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab";

    const INPUT: &str = include_str!("../test_data/day_02.txt");

    #[test_case(EXAMPLE_1 => 12)]
    #[test_case(INPUT => 5681)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    const EXAMPLE_2: &str = "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz";

    #[test_case(EXAMPLE_2 => "fgij")]
    #[test_case(INPUT => "uqyoeizfvmbistpkgnocjtwld")]
    fn part_2(input: &str) -> String {
        super::part_2(input)
    }
}
