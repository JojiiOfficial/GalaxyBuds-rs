use crate::model::Model;

use super::{
    bud_property::{match_site, Side},
    bytebuff::ByteBuff,
    ids,
    utils::byteutil,
    Payload,
};

#[derive(Debug, Copy, Clone)]
pub enum DebugVariant {
    SerialNumber,
    GetAllData,
    Sku,
}

/// Debug
#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Default, Clone)]
pub struct GetAllData {
    pub msg_version: u8,
    pub revision: u8,
    pub hw_version: String,
    pub bt_address_right: String,
    pub bt_address_left: String,
    pub proximity_left: i16,
    pub proximity_left_offset: i16,
    pub proximity_right: i16,
    pub proximity_right_offset: i16,
    pub thermistor_left: f32,
    pub thermistor_right: f32,
    pub adc_soc_left: i16,
    pub adc_vcell_left: f32,
    pub adc_current_left: f64,
    pub adc_soc_right: i16,
    pub adc_vcell_right: f32,
    pub adc_current_right: f64,
    pub gyro_left_x: i16,
    pub gyro_left_y: i16,
    pub gyro_left_z: i16,
    pub gyro_right_x: i16,
    pub gyro_right_y: i16,
    pub gyro_right_z: i16,
    pub cradle_batt_left: u8,
    pub cradle_batt_right: u8,
}

impl GetAllData {
    pub fn new(arr: &[u8], model: Model) -> Self {
        let buff = ByteBuff::new(arr);

        let hw_version = {
            let buff1 = buff.get(1);
            format!("rev{}{}", (buff1 & 240) >> 4, buff1 & 15)
        };

        if model == Model::Buds {
            unimplemented!();
        }

        let mut data = Self {
            msg_version: buff.get(0),
            revision: (buff.get(1) & 240) >> 4,
            hw_version,
            bt_address_left: buff.get_hex_str(6, 6).to_uppercase(),
            bt_address_right: buff.get_hex_str(12, 6).to_uppercase(),
            proximity_left: buff.get_short(30),
            proximity_left_offset: buff.get_short(32),
            proximity_right: buff.get_short(34),
            proximity_right_offset: buff.get_short(36),
            thermistor_left: buff.get_short(38) as f32 * 0.1_f32,
            thermistor_right: buff.get_short(40) as f32 * 0.1_f32,
            adc_soc_left: buff.get_short(42),
            adc_vcell_left: buff.get_short(44) as f32 * 0.01,
            adc_current_left: byteutil::calc_current(buff.get_short(46)),
            adc_soc_right: buff.get_short(48),
            adc_vcell_right: buff.get_short(50) as f32 * 0.01,
            adc_current_right: byteutil::calc_current(buff.get_short(52)),
            cradle_batt_left: buff.get(82),
            cradle_batt_right: buff.get(83),
            ..Default::default()
        };

        if model == Model::BudsLive {
            data.gyro_left_x = buff.get_short(84);
            data.gyro_left_y = buff.get_short(86);
            data.gyro_left_z = buff.get_short(88);
            data.gyro_right_x = buff.get_short(90);
            data.gyro_right_y = buff.get_short(92);
            data.gyro_right_z = buff.get_short(94);
        }

        data
    }

    // Return the bluetooth address of the given bud
    pub fn get_bt_address(&self, side: Side) -> &str {
        match_site(&self.bt_address_left, &self.bt_address_right, side)
    }

    // Get the proximity
    pub fn get_proximity(&self, side: Side) -> i16 {
        match_site(self.proximity_left, self.proximity_right, side)
    }

    // Get the proximity_offset
    pub fn get_proximity_offset(&self, side: Side) -> i16 {
        match_site(
            self.proximity_left_offset,
            self.proximity_right_offset,
            side,
        )
    }

    // Get thermistor
    pub fn get_thermistor(&self, side: Side) -> f32 {
        match_site(self.thermistor_left, self.thermistor_right, side)
    }

    // Get Adc SOC
    pub fn get_adc_soc(&self, side: Side) -> i16 {
        match_site(self.adc_soc_left, self.adc_soc_right, side)
    }

    // Get Adc vcell
    pub fn get_adc_vcell(&self, side: Side) -> f32 {
        match_site(self.adc_vcell_left, self.adc_vcell_right, side)
    }

    // Get Adc current
    pub fn get_adc_current(&self, side: Side) -> f64 {
        match_site(self.adc_current_left, self.adc_current_right, side)
    }

    // Get cradle battery
    pub fn get_cradle_battery(&self, side: Side) -> u8 {
        match_site(self.cradle_batt_left, self.cradle_batt_right, side)
    }

    // Get gyro x
    pub fn get_gyro_x(&self, side: Side) -> i16 {
        match_site(self.gyro_left_x, self.gyro_right_x, side)
    }

    // Get gyro y
    pub fn get_gyro_y(&self, side: Side) -> i16 {
        match_site(self.gyro_left_y, self.gyro_right_y, side)
    }

    // Get gyro z
    pub fn get_gyro_z(&self, side: Side) -> i16 {
        match_site(self.gyro_left_z, self.gyro_right_z, side)
    }
}

// Allow parsing Message to a GetAllData
impl Into<GetAllData> for super::Message {
    fn into(self) -> GetAllData {
        GetAllData::new(self.get_payload_bytes(), self.model)
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
