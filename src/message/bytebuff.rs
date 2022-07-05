use std::convert::TryInto;

use super::super::utils::byteutil;

pub(crate) struct ByteBuff<'a> {
    data: &'a [u8],
}

impl<'a> ByteBuff<'a> {
    // Create a new ByteBuff
    pub(crate) fn new(arr: &[u8]) -> ByteBuff {
        ByteBuff { data: arr }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub fn range(&self, offset: usize, len: usize) -> &[u8] {
        &self.data[offset..offset + len]
    }

    pub fn get_int(&self, offset: usize) -> u32 {
        let b: [u8; 4] = self.data[offset..offset + 4].try_into().unwrap();
        u32::from_le_bytes(b)
    }

    // Get a value at the given offset
    pub fn get(&self, offset: usize) -> u8 {
        self.data[offset]
    }

    // Return a short value starting from offset
    pub fn get_short(&self, offset: usize) -> i16 {
        byteutil::to_short(&self.data, offset)
    }

    // Get a bool value at the given offset
    pub fn get_bool(&self, offset: usize) -> bool {
        self.get(offset) == 1
    }

    pub fn bin_digit_val(&self, offset: usize, pos: usize) -> u8 {
        self.get(offset) & (1 << pos)
    }

    pub fn bin_digit_bool(&self, offset: usize, pos: usize) -> bool {
        self.get(offset) & (1 << pos) == (1 << pos)
    }

    pub fn get_hex_str(&self, offset: usize, len: usize) -> String {
        let mut s = String::new();

        for i in offset..offset + len {
            s.push_str(&format!("{:02x}", self.get(i)));

            if i != offset + len - 1 {
                s.push(':')
            }
        }

        s
    }
}
