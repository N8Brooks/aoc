pub fn part_1(input: &str) -> usize {
    design_counts(input).filter(|&count| count > 0).count()
}

pub fn part_2(input: &str) -> usize {
    design_counts(input).sum()
}

fn design_counts(input: &str) -> impl Iterator<Item = usize> + '_ {
    let (patterns, towels) = input.split_once("\n\n").unwrap();
    let trie = Trie::from_iter(patterns.split(", "));
    towels.lines().map(move |towel| {
        let mut counts = vec![0; towel.len() + 1];
        counts[0] = 1;
        for i in 0..towel.len() {
            let count = counts[i];
            if count == 0 {
                continue;
            }
            towel[i..]
                .bytes()
                .scan(&trie, |u, c| u[c].as_ref().inspect(|v| *u = v))
                .enumerate()
                .filter(|(_, u)| u.is_leaf)
                .for_each(|(j, _)| counts[i + j + 1] += count);
        }
        counts[towel.len()]
    })
}

#[derive(Debug, Default)]
struct Trie<'a> {
    children: [Option<Box<Trie<'a>>>; 26], // assumes lowercase ASCII
    is_leaf: bool,                         // assumes no duplicate patterns
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> Trie<'a> {
    fn insert<'b: 'a>(&mut self, s: &'b str) {
        s.bytes()
            .fold(self, |u, c| u[c].get_or_insert_default())
            .is_leaf = true;
    }
}

impl<'a> std::ops::Index<u8> for Trie<'a> {
    type Output = Option<Box<Trie<'a>>>;

    fn index(&self, c: u8) -> &Self::Output {
        let i = (c - b'a') as usize;
        &self.children[i]
    }
}

impl std::ops::IndexMut<u8> for Trie<'_> {
    fn index_mut(&mut self, c: u8) -> &mut Self::Output {
        let i = (c - b'a') as usize;
        &mut self.children[i]
    }
}

impl<'a, 'b: 'a> FromIterator<&'b str> for Trie<'a> {
    fn from_iter<I: IntoIterator<Item = &'b str>>(iter: I) -> Self {
        let mut trie = Trie::default();
        iter.into_iter().for_each(|s| trie.insert(s));
        trie
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_19.txt");

    const EXAMPLE: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test_case(EXAMPLE, 6)]
    #[test_case(INPUT, 272)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 16)]
    #[test_case(INPUT, 1041529704688380)]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
