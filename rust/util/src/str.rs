use crate::ascii;

use crate::array::transpose;

pub fn from_image<const N: usize>(&image: &[[u8; N]; 6]) -> String {
    transpose(image)
        .as_chunks()
        .0
        .iter()
        .map(|&cols| {
            let letter = transpose(cols);
            ascii::from_letter(&letter)
        })
        .collect()
}
