mod bytebuff;

pub mod ambient_mode;
pub mod anc_updated;
pub mod bud_property;
pub mod debug;
pub mod extended_status_updated;
pub mod find_my_bud;
pub mod fit_check;
pub mod ids;
pub mod lock_touchpad;
pub mod manager;
pub mod mute_earbud;
pub mod response;
pub mod set_noise_reduction;
pub mod set_touchpad_option;
pub mod simple;
pub mod status_updated;
pub mod touch_updated;
pub mod touchpad_action;
pub mod usage_report;
pub mod voice_wakeup_listening_status;

use crate::{
    model::Model,
    utils::{
        self, byteutil,
        crc16::{self, crc16_ccitt},
    },
};

/// End of message
pub const EOM: u8 = 221;
/// Begin of message
pub const BOM: u8 = 253;

/// Message contains the data of a message and
/// its into a `msg` trait implementing parsed
/// payload. In addition it contains some nice
/// functions which are dependend on the data
#[derive(Debug, Clone)]
pub struct Message {
    // the data of the message
    data: Vec<u8>,
    model: Model,
}

/// Msg defines the trait which need to be
/// implemented by an inner message (msg).
pub trait Payload {
    /// Getter for the message ID
    fn get_id(&self) -> u8;

    /// The payload data encoded for sending
    fn get_data(&self) -> Vec<u8> {
        vec![]
    }

    fn is_response(&self) -> bool {
        false
    }

    /// Create a message byte array from a message. This
    /// is required to send a message to the buds.
    fn to_byte_array(&self) -> Vec<u8> {
        let id = Self::get_id(&self);
        let payload_data = Self::get_data(&self);
        let payload_len = payload_data.len();

        let i2 = payload_len + 3;
        let i3 = i2 + 4;

        let mut b_arr: Vec<u8> = vec![0; i3];
        b_arr[0] = BOM;
        b_arr[i3 - 1 as usize] = EOM;

        let create_header = Self::create_header(self, i2 as i32);
        b_arr[1] = create_header[0];
        b_arr[2] = create_header[1];

        let mut b_arr2: Vec<u8> = vec![0; i2];
        b_arr2[0] = id;
        utils::array::arraycopy(&payload_data, 0, &mut b_arr2, 1, payload_data.len());

        let crc16_ccitt = crc16::crc16_ccitt(&b_arr2, b_arr2.len() - 2);

        let barr2_len = b_arr2.len();
        b_arr2[barr2_len - 2] = (crc16_ccitt & 255) as u8;
        b_arr2[barr2_len - 1] = ((crc16_ccitt >> 8) & 255) as u8;
        utils::array::arraycopy(&b_arr2, 0, &mut b_arr, 3, b_arr2.len());

        b_arr
    }

    /// Create a header for the message
    fn create_header(&self, i: i32) -> [u8; 2] {
        let mut from_short = byteutil::from_short(i & 1023);

        // use the msg's value here since its only used to send
        // messages and we want to have control over this
        // value from msg, not Message
        if self.is_response() {
            from_short[1] |= 16;
        }

        from_short
    }
}

impl Message {
    /// Create a new message object from read data
    #[inline]
    pub fn new<I: Into<Vec<u8>>>(data: I, model: Model) -> Message {
        Message {
            data: data.into(),
            model,
        }
    }

    /// Get the payload length of the message
    #[inline]
    pub fn get_payload_length(&self) -> i32 {
        self.get_u8() & 1023
    }

    /// Check whether the message is a fragment or not. Fragments seem
    /// only to be used in Fota messages
    #[inline]
    pub fn is_fragment(&self) -> bool {
        self.get_u8() & 8192 != 0
    }

    /// Checks if a message is a response
    #[inline]
    pub fn is_response(&self) -> bool {
        (self.get_u8() & 4096) != 0
    }

    /// Return the header of the message
    #[inline]
    pub fn get_u8(&self) -> i32 {
        (byteutil::to_u8(self.data[2]) << 8) + byteutil::to_u8(self.data[1])
    }

    /// Get the payload start index of the messages data
    #[inline]
    pub fn get_payload_start_index() -> usize {
        3
    }

    /// Return the bytes of the payload within the message
    #[inline]
    pub fn get_payload_bytes(&self) -> &[u8] {
        &self.data[Self::get_payload_start_index() + 1..]
    }

    /// Get the message id
    #[inline]
    pub fn get_id(&self) -> u8 {
        self.data[3]
    }

    /// Returns `true` if the given message really represents a message that should/can be parsed
    #[inline]
    pub fn is_message(&self) -> bool {
        self.data
            .get(self.get_payload_length() as usize + 3)
            .map(|i| *i == 221)
            .unwrap_or_default()
    }

    /// Verify that the message is correctly received using the last 2 bytes of the message as crc
    /// checksum
    pub fn check_crc(&self) -> bool {
        if self.data.len() < 5 {
            return false;
        }

        let mut b_arr = self.payload_with_chsum().to_vec();
        let l = b_arr.len();

        let b = b_arr[l - 1];
        b_arr[l - 1] = b_arr[b_arr.len() - 2];
        b_arr[l - 2] = b;

        crc16_ccitt(&b_arr, l) == 0
    }

    /// Returns the messages payload with the checksum
    #[inline]
    fn payload_with_chsum(&self) -> &[u8] {
        let start = Self::get_payload_start_index();
        let end = self.data.len() - 1;
        &self.data[start..end]
    }
}
