use itertools::{Either, Itertools as _};

pub fn part_1(input: &str) -> usize {
    let (locks, keys): (Vec<_>, Vec<_>) = input.split("\n\n").partition_map(|section| {
        // assume whole line is simialr
        let is_lock = section.starts_with('#');
        // assume there are always 5 counts with a max of 5
        let counts = section
            .lines()
            .skip(1)
            .take(5)
            .flat_map(|line| line.bytes().enumerate())
            .filter(|(_, c)| *c == b'#')
            .fold([0u8; 5], |mut counts, (j, _)| {
                counts[j] += 1;
                counts
            });
        if is_lock {
            Either::Left(counts)
        } else {
            Either::Right(counts)
        }
    });
    locks
        .into_iter()
        .cartesian_product(keys)
        .filter(|(lock, key)| lock.iter().zip(key).all(|(&lock, &key)| lock <= 5 - key))
        .count()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    const INPUT: &str = include_str!("../test_data/day_25.txt");

    #[test_case(EXAMPLE, 3)]
    #[test_case(INPUT, 3525)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }
}
