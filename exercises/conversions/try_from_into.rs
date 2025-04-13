// try_from_into.rs
//
// TryFrom is a simple and safe type conversion that may fail in a controlled
// way under some circumstances. Basically, this is the same as From. The main
// difference is that this should return a Result type instead of the target
// type itself. You can read more about it at
// https://doc.rust-lang.org/std/convert/trait.TryFrom.html
//
// Execute `rustlings hint try_from_into` or use the `hint` watch subcommand for
// a hint.


use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}


#[derive(Debug, PartialEq)]
enum IntoColorError {
    
    BadLen,
    
    IntConversion,
}


impl TryFrom<&[i16]> for Color {
    type Error = IntoColorError;

    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        if slice.len() != 3 {
            return Err(IntoColorError::BadLen);
        }

        let [r, g, b] = [slice[0], slice[1], slice[2]];

        if r < 0 || r > 255 || g < 0 || g > 255 || b < 0 || b > 255 {
            return Err(IntoColorError::IntConversion);
        }

        Ok(Color {
            red: r as u8,
            green: g as u8,
            blue: b as u8,
        })
    }
}
