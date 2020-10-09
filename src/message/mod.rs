pub mod bud_property;

pub mod extended_status_updated;
pub mod ids;
pub mod mute_earbud;
pub mod set_noise_reduction;
pub mod simple;
pub mod status_updated;
pub mod touch_updated;
pub mod voice_wakeup_listening_status;

use crate::utils::{byteutil, crc16};

pub const EOM: u8 = 221;

pub trait Message {
    fn get_id(&self) -> u8;

    fn get_data(&self) -> Vec<u8> {
        vec![]
    }

    fn to_byte_array(&self) -> Vec<u8> {
        let b = Self::get_id(&self);
        let data = Self::get_data(self);

        let i = {
            if data.len() > 0 {
                data.len()
            } else {
                0
            }
        };

        let i2 = i + 1 + 2;
        let i3 = i2 + 3 + 1;

        let mut b_arr: Vec<u8> = vec![0; i3];
        b_arr[0] = 253;
        b_arr[i3 - 1 as usize] = EOM;

        let create_header = create_header(i2 as i32);
        b_arr[1] = create_header[0];
        b_arr[2] = create_header[1];

        let mut b_arr2: Vec<u8> = vec![0; i2];
        b_arr2[0] = b;
        arraycopy(&data, 0, &mut b_arr2, 1, data.len());

        let crc16_ccitt = crc16::crc16_ccitt(&b_arr2, b_arr2.len() - 1);

        let barr2_len = b_arr2.len();
        b_arr2[barr2_len - 2] = (crc16_ccitt & 255) as u8;
        b_arr2[barr2_len - 1] = ((crc16_ccitt >> 8) & 255) as u8;
        arraycopy(&b_arr2, 0, &mut b_arr, 3, b_arr2.len());

        b_arr
    }
}

fn create_header(i: i32) -> [u8; 2] {
    let from_short = byteutil::from_short(i & 1023);
    from_short
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
