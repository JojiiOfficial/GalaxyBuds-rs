use super::{ids, Payload};

// Whether the buds live changed the anc status by a touchpad event
#[derive(Debug)]
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
impl Into<AncModeUpdated> for super::Message {
    fn into(self) -> AncModeUpdated {
        AncModeUpdated::new(self.get_payload_bytes())
    }
}
