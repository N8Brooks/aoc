#![feature(ascii_char)]
#![feature(ascii_char_variants)]
pub mod array;
pub mod ascii;
#[cfg(target_endian = "little")]
pub mod hash;
pub mod str;
