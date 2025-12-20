use super::{ids, Payload};

// Whether the buds live changed the anc status by a touchpad event
#[derive(Debug, Clone, Copy)]
pub struct AncModeUpdated {
    pub anc_enabled: bool,
}

impl AncModeUpdated {
    pub fn new(arr: &[u8]) -> Self {
        Self {
            anc_enabled: arr[0] == 1,
        }
    }
}

impl Payload for AncModeUpdated {
    fn get_id(&self) -> u8 {
        ids::NOISE_REDUCTION_MODE_UPDATE
    }
}

// Allow parsing Message to a StatusUpdate
impl From<super::Message> for AncModeUpdated {
    fn from(value: super::Message) -> Self {
        AncModeUpdated::new(value.get_payload_bytes())
    }
}
