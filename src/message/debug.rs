use super::{bytebuff::ByteBuff, ids, utils::byteutil, Payload};

#[derive(Debug, Copy, Clone)]
pub enum DebugVariant {
    SerialNumber,
    GetAllData,
    Sku,
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
        }
    }
}

#[derive(Debug)]
pub struct GetAllData {
    msg_version: u8,
    revision: u8,
    hw_version: String,
    bt_address_right: String,
    bt_address_left: String,
    proximity_left: i16,
    proximity_left_offset: i16,
    proximity_right: i16,
    proximity_right_offset: i16,
    thermistor_left: f32,
    thermistor_right: f32,
    adc_soc_left: i16,
    adc_vcell_left: f64,
    adc_current_left: f64,
    adc_soc_right: i16,
    adc_vcell_right: f64,
    adc_current_right: f64,
    gyro_left_0: i16,
    gyro_left_1: i16,
    gyro_left_2: i16,
    gyro_right_0: i16,
    gyro_right_1: i16,
    gyro_right_2: i16,
    cradle_batt_left: u8,
    cradle_batt_right: u8,
}

impl GetAllData {
    pub fn new(arr: &[u8]) -> Self {
        let buff = ByteBuff::new(arr);

        let hw_version = {
            let buff1 = buff.get(1);
            format!("rev{}{}", (buff1 & 240) >> 4, buff1 & 15).to_string()
        };

        Self {
            msg_version: buff.get(0),
            revision: (buff.get(1) & 240) >> 4,
            hw_version,
            bt_address_left: buff.get_hex_str(6, 6).to_uppercase(),
            bt_address_right: buff.get_hex_str(12, 6).to_uppercase(),
            proximity_left: buff.get_short(30),
            proximity_left_offset: buff.get_short(32),
            proximity_right: buff.get_short(34),
            proximity_right_offset: buff.get_short(36),
            thermistor_left: buff.get_short(38) as f32 * 0.1,
            thermistor_right: buff.get_short(40) as f32 * 0.1,
            adc_soc_left: buff.get_short(42),
            adc_vcell_left: buff.get_short(44) as f64 * 0.01,
            adc_current_left: byteutil::calc_current(buff.get_short(46)),
            adc_soc_right: buff.get_short(48),
            adc_vcell_right: buff.get_short(50) as f64 * 0.01,
            adc_current_right: byteutil::calc_current(buff.get_short(52)),
            cradle_batt_left: buff.get(82),
            cradle_batt_right: buff.get(83),
            gyro_left_0: buff.get_short(84),
            gyro_left_1: buff.get_short(86),
            gyro_left_2: buff.get_short(88),
            gyro_right_0: buff.get_short(90),
            gyro_right_1: buff.get_short(92),
            gyro_right_2: buff.get_short(94),
        }
    }
}

// Allow parsing Message to a GetAllData
impl Into<GetAllData> for super::Message {
    fn into(self) -> GetAllData {
        GetAllData::new(self.get_payload_bytes())
    }
}

#[derive(Debug)]
pub struct SerialNumber {
    serial_number_left: String,
    serial_number_right: String,
}

impl SerialNumber {
    pub fn new(arr: &[u8]) -> Self {
        SerialNumber {
            serial_number_left: byteutil::to_serial_number(&arr, 0, 11),
            serial_number_right: byteutil::to_serial_number(&arr, 11, 11),
        }
    }
}

// Allow parsing Message to a SerialNumber
impl Into<SerialNumber> for super::Message {
    fn into(self) -> SerialNumber {
        SerialNumber::new(self.get_payload_bytes())
    }
}

#[derive(Debug)]
pub struct Sku {
    sku_left: String,
    sku_right: String,
}

impl Sku {
    pub fn new(arr: &[u8]) -> Self {
        Sku {
            sku_left: byteutil::to_serial_number(&arr, 0, 14),
            sku_right: byteutil::to_serial_number(&arr, 14, 14),
        }
    }
}

// Allow parsing Message to a SerialNumber
impl Into<Sku> for super::Message {
    fn into(self) -> Sku {
        Sku::new(self.get_payload_bytes())
    }
}
