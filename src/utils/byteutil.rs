pub fn to_u8(b: u8) -> i32 {
    b as i32 & 255
}

pub fn value_of_left(b: u8) -> u8 {
    (b & 240) >> 4
}

pub fn value_of_right(b: u8) -> u8 {
    b & 15
}

pub fn from_short(i: i32) -> [u8; 2] {
    [i as u8, (i >> 8) as u8]
}
