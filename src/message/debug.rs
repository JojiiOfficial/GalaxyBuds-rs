use super::{bytebuff::ByteBuff, ids, utils::byteutil, Payload};

#[derive(Debug, Copy, Clone)]
pub enum DebugVariant {
    SerialNumber,
    GetAllData,
    BuildInfo,
    Sku,
    PeRssi,
}

/// Debug
#[derive(Debug)]
pub struct Debug {
    variant: DebugVariant,
}

pub fn new(variant: DebugVariant) -> Debug {
    Debug { variant }
}

impl Payload for Debug {
    fn get_data(&self) -> Vec<u8> {
        vec![]
    }

    fn get_id(&self) -> u8 {
        match self.variant {
            DebugVariant::SerialNumber => ids::DEBUG_SERIAL_NUMBER,
            DebugVariant::GetAllData => ids::DEBUG_GET_ALL_DATA,
            DebugVariant::Sku => ids::DEBUG_SKU,
            _ => 0,
        }
    }
}

#[derive(Debug)]
pub struct GetAllData {
    msg_version: u8,
    revision: u8,
    bt_address_right: String,
    bt_address_left: String,
    proxymity_left: i16,
    proxymity_left_offset: i16,
    proxymity_right: i16,
    proxymity_right_offset: i16,
    battery_left_0: u8,
    battery_left_1: u8,
    battery_left_2: u8,
    battery_right_0: u8,
    battery_right_1: u8,
    battery_right_2: u8,
    cradle_batt_left: u8,
    cradle_batt_right: u8,
    has_gyro: bool,
}

impl GetAllData {
    pub fn new(arr: &[u8]) -> Self {
        let buff = ByteBuff::new(&arr);

        // 18
        Self {
            msg_version: buff.get(0),
            revision: (buff.get(1) & 240) >> 4,
            bt_address_left: buff.get_hex_str(6, 6).to_uppercase(),
            bt_address_right: buff.get_hex_str(12, 6).to_uppercase(),
            proxymity_left: buff.get_short(20),
            proxymity_left_offset: buff.get_short(22),
            proxymity_right: buff.get_short(24),
            proxymity_right_offset: buff.get_short(26),
            battery_left_0: arr[20],
            battery_left_1: arr[21],
            battery_left_2: arr[22],
            battery_right_0: arr[23],
            battery_right_1: arr[24],
            battery_right_2: arr[25],
            cradle_batt_left: buff.get(74),
            cradle_batt_right: buff.get(75),
            has_gyro: arr[0] >= 0,
        }
    }
}

// Allow parsing Message to a StatusUpdate
impl Into<GetAllData> for super::Message {
    fn into(self) -> GetAllData {
        GetAllData::new(self.get_payload_bytes())
    }
}
