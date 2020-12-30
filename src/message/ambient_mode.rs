use super::{ids, Payload};

// Whether the buds enabled the ambient_mode themselves.
#[derive(Debug, Clone, Copy)]
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

// Set the ambient volume level
#[derive(Debug, Copy, Clone)]
pub struct SetAmbientVolume {
    pub new_volume_lvl: u8,
}

impl SetAmbientVolume {
    pub fn new(new_volume_lvl: u8) -> Self {
        Self { new_volume_lvl }
    }
}

impl Payload for SetAmbientVolume {
    fn get_id(&self) -> u8 {
        ids::AMBIENT_VOLUME
    }

    fn get_data(&self) -> Vec<u8> {
        // 1 is lowest, 4 is highest.
        // However the buds expect it from 0-3.
        vec![self.new_volume_lvl - 1]
    }
}

// Set the ambient mode: enabled or disabled
#[derive(Debug, Copy, Clone)]
pub struct SetAmbientMode {
    pub enabled: bool,
}

impl SetAmbientMode {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}

impl Payload for SetAmbientMode {
    fn get_id(&self) -> u8 {
        ids::SET_AMBIENT_MODE
    }

    fn get_data(&self) -> Vec<u8> {
        vec![{
            if self.enabled {
                1
            } else {
                0
            }
        }]
    }
}

// Set the extra high ambient mode: enabled or disabled
#[derive(Debug, Copy, Clone)]
pub struct SetExtraHighVolume {
    pub enabled: bool,
}

impl SetExtraHighVolume {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}

impl Payload for SetExtraHighVolume {
    fn get_id(&self) -> u8 {
        ids::EXTRA_HIGH_AMBIENT
    }

    fn get_data(&self) -> Vec<u8> {
        vec![{
            if self.enabled {
                1
            } else {
                0
            }
        }]
    }
}
