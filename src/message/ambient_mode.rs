use super::{ids, Payload};

// Whether the buds enabled the ambient_mode themselves.
#[derive(Debug)]
pub struct AmbientModeUpdated {
    pub ambient_mode: bool,
}

impl AmbientModeUpdated {
    pub fn new(arr: &[u8]) -> Self {
        Self {
            ambient_mode: arr[0] == 1,
        }
    }
}

impl Payload for AmbientModeUpdated {
    fn get_id(&self) -> u8 {
        ids::AMBIENT_MODE_UPDATED
    }
}

// Allow parsing Message to a StatusUpdate
impl Into<AmbientModeUpdated> for super::Message {
    fn into(self) -> AmbientModeUpdated {
        AmbientModeUpdated::new(self.get_payload_bytes())
    }
}

// The ambient volume level
#[derive(Debug)]
pub struct AmbientVolume {
    pub volume: i32,
}

impl AmbientVolume {
    pub fn new(arr: &[u8]) -> Self {
        Self {
            volume: arr[0] as i32,
        }
    }
}

impl Payload for AmbientVolume {
    fn get_id(&self) -> u8 {
        ids::AMBIENT_VOLUME
    }
}

// Allow parsing Message to a StatusUpdate
impl Into<AmbientVolume> for super::Message {
    fn into(self) -> AmbientVolume {
        AmbientVolume::new(self.get_payload_bytes())
    }
}
