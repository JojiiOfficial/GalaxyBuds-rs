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

pub fn to_short(arr: &[u8], offset: usize) -> i16 {
    ((arr[offset] as i16 & 0xFF) << 8) | (arr[offset + 1] as i16 & 0xFF)
}

pub fn as_batt2(byte: i16) -> f64 {
    let float = byte as f64 * 1.0E-4;
    let formatted = format!("{}", float);

    if formatted.len() > 6 {
        formatted[0..6].to_string().parse::<f64>().unwrap_or(float)
    } else {
        float
    }
}

pub fn to_serial_number(arr: &[u8], offset: usize) -> String {
    let mut sn_arr: [u8; 11] = [0; 11];

    for i in 0..11 {
        sn_arr[i] = arr[i + offset];
        if sn_arr[i] == 0 {
            return "".to_string();
        }
    }

    String::from_utf8(sn_arr.into()).unwrap_or_default()
}
