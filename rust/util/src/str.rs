use crate::{array::Transpose as _, ascii};

pub fn from_image<const N: usize>(&image: &[[u8; N]; 6]) -> String {
    image
        .transpose()
        .as_chunks()
        .0
        .iter()
        .map(|&cols| ascii::from_letter(&cols.transpose()))
        .collect()
}
