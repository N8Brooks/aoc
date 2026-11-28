use crate::ascii;

pub fn from_image<const N: usize>(image: &[[u8; N]; 6]) -> String {
    let rows = image.each_ref().map(|row| row.as_chunks::<5>().0);

    (0..N / 5)
        .map(|i| ascii::from_letter(rows.map(|row| &row[i])))
        .collect()
}
