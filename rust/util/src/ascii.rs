use std::ascii::Char::{self, *};

pub fn from_letter(letter: &[[u8; 5]; 6]) -> Char {
    match letter.each_ref() {
        #[rustfmt::skip]
        [
            b" ##  ",
            b"#  # ",
            b"#  # ",
            b"#### ",
            b"#  # ",
            b"#  # ",
        ] => CapitalA,
        #[rustfmt::skip]
        [
            b" ##  ",
            b"#  # ",
            b"#    ",
            b"#    ",
            b"#  # ",
            b" ##  ",
        ] => CapitalC,
        #[rustfmt::skip]
        [
            b"#### ",
            b"#    ",
            b"###  ",
            b"#    ",
            b"#    ",
            b"#### ",
        ] => CapitalE,
        #[rustfmt::skip]
        [
            b" ### ",
            b"  #  ",
            b"  #  ",
            b"  #  ",
            b"  #  ",
            b" ### ",
        ] => CapitalI,
        #[rustfmt::skip]
        [
            b"#    ",
            b"#    ",
            b"#    ",
            b"#    ",
            b"#    ",
            b"#### ",
        ] => CapitalL,
        #[rustfmt::skip]
        [
            b" ##  ",
            b"#  # ",
            b"#  # ",
            b"#  # ",
            b"#  # ",
            b" ##  ",
        ] => CapitalO,
        #[rustfmt::skip]
        [
            b"###  ",
            b"#  # ",
            b"#  # ",
            b"###  ",
            b"#    ",
            b"#    ",
        ] => CapitalP,
        #[rustfmt::skip]
        [
            b"###  ",
            b"#  # ",
            b"#  # ",
            b"###  ",
            b"# #  ",
            b"#  # ",
        ] => CapitalR,
        #[rustfmt::skip]
        [
            b"#  # ",
            b"#  # ",
            b"#  # ",
            b"#  # ",
            b"#  # ",
            b" ##  ",
        ] => CapitalU,
        rows => {
            let lines = rows
                .map(|row| format!("            b\"{}\",", str::from_utf8(row).unwrap()))
                .join("\n");
            panic!(
                "Unknown letter:
        #[rustfmt::skip]
        [
{lines}
        ] => QuestionMark,",
            )
        }
    }
}
