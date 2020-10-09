pub fn toU8(b: u8) -> i32 {
    b as i32 & 255
}

pub fn value_of_left(b: u8) -> u8 {
    (b & 240) >> 4
}

pub fn value_of_right(b: u8) -> u8 {
    b & 15
}
