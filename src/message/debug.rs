use super::{ids, Payload};

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
