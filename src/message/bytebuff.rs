use super::super::utils::byteutil;

pub(crate) struct ByteBuff<'a> {
    data: &'a [u8],
}

impl<'a> ByteBuff<'a> {
    // Create a new ByteBuff
    pub(crate) fn new(arr: &[u8]) -> ByteBuff {
        ByteBuff { data: arr }
    }

    // Get a value at the given offset
    pub fn get(&self, offset: usize) -> u8 {
        self.data[offset]
    }

    // Return a short value starting from offset
    pub fn get_short(&self, offset: usize) -> i16 {
        byteutil::to_short(&self.data, offset)
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
