use itertools::Itertools as _;

pub fn part_1<const M: usize, const N: usize>(input: &str) -> usize {
    let layer = input
        .as_bytes()
        .chunks_exact(M * N)
        .min_by_key(|layer| layer.iter().filter(|&&b| b == b'0').count())
        .unwrap();
    let count_1 = layer.iter().filter(|&&b| b == b'1').count();
    let count_2 = layer.iter().filter(|&&b| b == b'2').count();
    count_1 * count_2
}

pub fn part_2<const M: usize, const N: usize>(input: &str) -> String {
    let (rows, []) = input.as_bytes().as_chunks::<N>() else {
        panic!("Input length is not a multiple of N");
    };
    let (layers, []) = rows.as_chunks::<M>() else {
        panic!("Input length is not a multiple of M * N");
    };
    let image = layers.iter().fold([[b'2'; N]; M], |mut layer_1, layer_2| {
        for (row_1, row_2) in layer_1.iter_mut().zip(layer_2) {
            for (pxl_1, pxl_2) in row_1.iter_mut().zip(row_2) {
                if *pxl_1 == b'2' {
                    *pxl_1 = *pxl_2;
                }
            }
        }
        layer_1
    });
    image
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|pxl| match pxl {
                    b'0' => ' ',
                    b'1' => '#',
                    _ => panic!("Invalid pixel value {pxl}"),
                })
                .collect::<String>()
        })
        .join("\n")
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE_1: &str = "123456789012";

    const INPUT: &str = include_str!("../test_data/day_08.txt");

    #[test_case(EXAMPLE_1, 1)]
    fn example_1(input: &str, expected: usize) {
        assert_eq!(super::part_1::<2, 3>(input), expected);
    }

    #[test_case(INPUT, 1920)]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1::<6, 25>(input), expected);
    }

    const EXAMPLE_2: &str = "0222112222120000";

    #[test_case(EXAMPLE_2, " #\n# ")]
    fn example_2(input: &str, expected: &str) {
        assert_eq!(super::part_2::<2, 2>(input), expected.to_string());
    }

    const EXPECTED_2: &str = "\
###   ##  #  # #     ##  
#  # #  # #  # #    #  # 
#  # #    #  # #    #  # 
###  #    #  # #    #### 
#    #  # #  # #    #  # 
#     ##   ##  #### #  # ";

    #[test_case(INPUT, EXPECTED_2)]
    fn part_2(input: &str, expected: &str) {
        assert_eq!(super::part_2::<6, 25>(input), expected.to_string());
    }
}
