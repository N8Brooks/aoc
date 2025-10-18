use itertools::Itertools as _;

pub fn part_1(input: &str, m: usize, n: usize) -> usize {
    let layer = input
        .as_bytes()
        .chunks_exact(m * n)
        .min_by_key(|layer| layer.iter().filter(|&&b| b == b'0').count())
        .unwrap();
    let count_1 = layer.iter().filter(|&&b| b == b'1').count();
    let count_2 = layer.iter().filter(|&&b| b == b'2').count();
    count_1 * count_2
}

pub fn part_2(input: &str, m: usize, n: usize) -> String {
    input
        .as_bytes()
        .chunks_exact(m * n)
        .fold(vec![b'2'; m * n], |mut layer_1, layer_2| {
            for (pxl_1, pxl_2) in layer_1.iter_mut().zip(layer_2) {
                if *pxl_1 == b'2' {
                    *pxl_1 = *pxl_2;
                }
            }
            layer_1
        })
        .chunks_exact(n)
        .map(|row| -> String {
            row.iter()
                .map(|pxl| match pxl {
                    b'0' => ' ',
                    b'1' => '#',
                    _ => panic!("Invalid pixel value {pxl}"),
                })
                .collect()
        })
        .join("\n")
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE_1: &str = "123456789012";

    const INPUT: &str = include_str!("../test_data/day_08.txt");

    #[test_case(EXAMPLE_1, 1, 2, 3)]
    #[test_case(INPUT, 1920, 6, 25)]
    fn part_1(input: &str, expected: usize, m: usize, n: usize) {
        assert_eq!(super::part_1(input, m, n), expected);
    }

    const EXAMPLE_2: &str = "0222112222120000";

    const EXPECTED_2: &str = "\
###   ##  #  # #     ##  
#  # #  # #  # #    #  # 
#  # #    #  # #    #  # 
###  #    #  # #    #### 
#    #  # #  # #    #  # 
#     ##   ##  #### #  # ";

    #[test_case(EXAMPLE_2, " #\n# ", 2, 2)]
    #[test_case(INPUT, EXPECTED_2, 6, 25)]
    fn part_2(input: &str, expected: &str, m: usize, n: usize) {
        assert_eq!(super::part_2(input, m, n), expected.to_string());
    }
}
