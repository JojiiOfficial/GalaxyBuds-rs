use super::{ids, Payload};

#[derive(Debug, Clone, Copy)]
pub struct TouchUpdated {
    pub status: bool,
}

impl TouchUpdated {
    /// New touch updated payload
    pub fn new(arr: &[u8]) -> TouchUpdated {
        TouchUpdated {
            status: arr[0] == 1,
        }
    }
}

impl Payload for TouchUpdated {
    fn get_id(&self) -> u8 {
        ids::TOUCH_UPDATED
    }
}

// Allow Into TouchUpdated from message
impl From<super::Message> for TouchUpdated {
    fn from(value: super::Message) -> Self {
        TouchUpdated::new(value.get_payload_bytes())
    }
}
