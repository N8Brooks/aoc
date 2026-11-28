use std::ascii::Char::{self, *};

pub fn from_letter(letter: [&[u8; 5]; 6]) -> Char {
    match letter {
        #[rustfmt::skip]
        [
            b".##..",
            b"#..#.",
            b"#..#.",
            b"####.",
            b"#..#.",
            b"#..#.",
        ] => CapitalA,

        #[rustfmt::skip]
        [
            b"###..",
            b"#..#.",
            b"###..",
            b"#..#.",
            b"#..#.",
            b"###..",
        ] => CapitalB,

        #[rustfmt::skip]
        [
            b".##..",
            b"#..#.",
            b"#....",
            b"#....",
            b"#..#.",
            b".##..",
        ] => CapitalC,

        #[rustfmt::skip]
        [
            b"####.", 
            b"#....",
            b"###..",
            b"#....",
            b"#....",
            b"####."
        ] => CapitalE,

        #[rustfmt::skip]
        [
            b"####.",
            b"#....",
            b"###..",
            b"#....",
            b"#....",
            b"#....",
        ] => CapitalF,

        #[rustfmt::skip]
        [
            b".##..",
            b"#..#.",
            b"#....",
            b"#.##.",
            b"#..#.",
            b".###.",
        ] => CapitalG,

        #[rustfmt::skip]
        [
            b"#..#.",
            b"#..#.",
            b"####.",
            b"#..#.",
            b"#..#.",
            b"#..#.",
        ] => CapitalH,

        #[rustfmt::skip]
        [
            b".###.",
            b"..#..",
            b"..#..",
            b"..#..",
            b"..#..",
            b".###.",
        ] => CapitalI,

        #[rustfmt::skip]
        [
            b"..##.",
            b"...#.",
            b"...#.",
            b"...#.",
            b"#..#.",
            b".##..",
        ] => CapitalJ,

        #[rustfmt::skip]
        [
            b"#..#.",
            b"#.#..",
            b"##...",
            b"#.#..",
            b"#.#..",
            b"#..#.",
        ] => CapitalK,

        #[rustfmt::skip]
        [
            b"#....",
            b"#....",
            b"#....",
            b"#....",
            b"#....",
            b"####.",
        ] => CapitalL,

        #[rustfmt::skip]
        [
            b".##..",
            b"#..#.",
            b"#..#.",
            b"#..#.",
            b"#..#.",
            b".##..",
        ] => CapitalO,

        #[rustfmt::skip]
        [
            b"###..",
            b"#..#.",
            b"#..#.",
            b"###..",
            b"#....",
            b"#....",
        ] => CapitalP,

        #[rustfmt::skip]
        [
            b"###..",
            b"#..#.",
            b"#..#.",
            b"###..",
            b"#.#..",
            b"#..#.",
        ] => CapitalR,

        #[rustfmt::skip]
        [
            b".###.",
            b"#....",
            b"#....",
            b".##..",
            b"...#.",
            b"###..",
        ] => CapitalS,

        #[rustfmt::skip]
        [
            b"#..#.",
            b"#..#.",
            b"#..#.",
            b"#..#.",
            b"#..#.",
            b".##..",
        ] => CapitalU,

        #[rustfmt::skip]
        [
            b"#...#",
            b"#...#",
            b".#.#.",
            b"..#..",
            b"..#..",
            b"..#..",
        ] => CapitalY,

        #[rustfmt::skip]
        [
            b"####.",
            b"...#.",
            b"..#..",
            b".#...",
            b"#....",
            b"####.",
        ] => CapitalZ,

        rows => {
            let rows = rows
                .map(|row| format!("            b{:?},", String::from_utf8_lossy(row)))
                .join("\n");

            panic!(
                "missing letter:
        #[rustfmt::skip]
        [
{rows}
        ] => todo!(),"
            )
        }
    }
}
