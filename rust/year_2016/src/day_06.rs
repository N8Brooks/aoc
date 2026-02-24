pub fn part_1(input: &str) -> String {
    counts(input)
        .into_iter()
        .map(|counts| {
            let (i, _) = counts
                .into_iter()
                .enumerate()
                .max_by_key(|&(_, c)| c)
                .unwrap();
            char::from(b'a' + u8::try_from(i).unwrap())
        })
        .collect()
}

pub fn part_2(input: &str) -> String {
    counts(input)
        .into_iter()
        .map(|counts| {
            let (i, _) = counts
                .into_iter()
                .enumerate()
                .filter(|&(_, c)| c > 0)
                .min_by_key(|&(_, c)| c)
                .unwrap();
            char::from(b'a' + u8::try_from(i).unwrap())
        })
        .collect()
}

pub fn counts(input: &str) -> Vec<[usize; 26]> {
    let n = input.find('\n').unwrap_or(input.len());
    input.lines().fold(vec![[0; 26]; n], |mut counts, line| {
        counts.iter_mut().zip(line.bytes()).for_each(|(c, b)| {
            let i: usize = (b - b'a').into();
            c[i] += 1;
        });
        counts
    })
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_06.txt");

    const EXAMPLE: &str = "eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";

    #[test_case(EXAMPLE => "easter")]
    #[test_case(INPUT => "gyvwpxaz")]
    fn part_1(input: &str) -> String {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => "advent")]
    #[test_case(INPUT => "jucfoary")]
    fn part_2(input: &str) -> String {
        super::part_2(input)
    }
}
