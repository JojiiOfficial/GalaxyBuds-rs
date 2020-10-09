// Message ID: 145 (-111)
use super::super::utils::{byteutil, crc16};

#[derive(Debug)]
pub struct SetNoiseReduction {
    pub noise_reduction: bool,
}

pub fn new(noise_reduction: bool) -> SetNoiseReduction {
    SetNoiseReduction { noise_reduction }
}

pub const EOM: u8 = 221;
impl SetNoiseReduction {
    pub fn get_data(&self) -> [u8; 1] {
        [self.noise_reduction.into()]
    }

    pub fn get_id() -> u8 {
        152
    }

    pub fn to_byte_array(&self) -> Vec<u8> {
        let b = Self::get_id();
        let data = self.get_data();

        let i = data.len();
        let i2 = i + 1 + 2;
        let i3 = i2 + 3 + 1;

        let mut bArr: Vec<u8> = vec![0; i3];
        bArr[0] = 253;
        bArr[i3 - 1 as usize] = EOM;

        let create_header = Self::create_header(i2 as i32);
        bArr[1] = create_header[0];
        bArr[2] = create_header[1];

        let mut bArr2: Vec<u8> = vec![0; i2];
        bArr2[0] = b;
        arraycopy(&data.into(), 0, &mut bArr2, 1, data.len());

        let barr2_len = bArr2.len();

        let crc16_ccitt = crc16::crc16_ccitt(&bArr2, barr2_len - 1);
        bArr2[barr2_len - 2] = (crc16_ccitt & 255) as u8;
        bArr2[barr2_len - 1] = ((crc16_ccitt >> 8) & 255) as u8;

        arraycopy(&bArr2, 0, &mut bArr, 3, bArr2.len());

        bArr
    }

    pub fn create_header(i: i32) -> [u8; 2] {
        let from_short = byteutil::from_short(i & 1023);
        from_short
    }
}

pub fn arraycopy<T>(
    src: &Vec<T>,
    src_pos: usize,
    dest: &mut Vec<T>,
    dest_post: usize,
    length: usize,
) where
    T: Copy + Default,
{
    if length + dest_post > dest.len() {
        dest.resize(length + dest_post, T::default());
    }

    for i in 0..length {
        dest[i + dest_post] = src[i + src_pos];
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_arraycopy() {
        let src = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
        let mut dest = vec![15, 25, 35, 45, 55, 65, 75, 85, 95, 105];

        super::arraycopy(&src, 3, &mut dest, 5, 4);

        assert_eq!(dest, vec![15, 25, 35, 45, 55, 40, 50, 60, 70, 105]);
    }
}
