use util::str;

pub fn part_1(input: &str, m: usize, n: usize) -> usize {
    let [_, ones, twos, ..] = input
        .as_bytes()
        .chunks_exact(m * n)
        .map(|layer| {
            layer.iter().fold([0; 10], |mut counts, &b| {
                let i = (b - b'0') as usize;
                counts[i] += 1;
                counts
            })
        })
        .min_by_key(|layer| layer[0])
        .unwrap();
    ones * twos
}

pub fn part_2<const N: usize>(input: &str) -> String
where
    [(); 6 * N]:,
{
    let image = visible_image::<6, N>(input);
    str::from_image(&image)
}

pub fn visible_image<const M: usize, const N: usize>(input: &str) -> [[u8; N]; M]
where
    [(); M * N]:,
{
    let pixels =
        input
            .as_bytes()
            .chunks_exact(M * N)
            .fold([b'2'; M * N], |mut layer_1, layer_2| {
                for (pxl_1, pxl_2) in layer_1.iter_mut().zip(layer_2) {
                    if *pxl_1 == b'2' {
                        *pxl_1 = *pxl_2;
                    }
                }
                layer_1
            });
    TryInto::<[[u8; N]; M]>::try_into(pixels.as_chunks().0)
        .unwrap()
        .map(|row| {
            row.map(|pxl| match pxl {
                b'0' => b' ',
                b'1' => b'#',
                _ => panic!("Invalid pixel value {pxl}"),
            })
        })
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE_1: &str = "123456789012";

    const INPUT: &str = include_str!("../test_data/day_08.txt");

    #[test_case(EXAMPLE_1, 2, 3 => 1)]
    #[test_case(INPUT, 6, 25 => 1920)]
    fn part_1(input: &str, m: usize, n: usize) -> usize {
        super::part_1(input, m, n)
    }

    #[test_case(INPUT => "PCULA")]
    fn part_2(input: &str) -> String {
        super::part_2::<25>(input)
    }

    #[test_case("0222112222120000" => [*b" #", *b"# "])]
    fn example_2(input: &str) -> [[u8; 2]; 2] {
        super::visible_image::<2, 2>(input)
    }
}
