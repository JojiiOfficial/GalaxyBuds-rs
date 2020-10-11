use super::{ids, Payload};

#[derive(Debug)]
pub struct TouchUpdated {
    pub status: bool,
}

/// New touch updated payload
pub fn new(arr: &[u8]) -> TouchUpdated {
    TouchUpdated {
        status: arr[0] == 1,
    }
}

impl Payload for TouchUpdated {
    fn get_id(&self) -> u8 {
        ids::TOUCH_UPDATED
    }
}

// Allow Into TouchUpdated from message
impl Into<TouchUpdated> for super::Message {
    fn into(self) -> TouchUpdated {
        new(self.get_payload_bytes())
    }
}
