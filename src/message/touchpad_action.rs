use super::{bud_property, ids, Payload};

#[derive(Debug, Clone, Copy)]
pub struct TouchAction {
    pub side: bud_property::Side,
    pub touch_count: u8,
}

/// New touch updated payload
pub fn new(arr: &[u8]) -> TouchAction {
    TouchAction {
        side: {
            if arr[0] == 1 {
                bud_property::Side::Left
            } else {
                bud_property::Side::Right
            }
        },
        touch_count: arr[1],
    }
}

impl Payload for TouchAction {
    fn get_id(&self) -> u8 {
        ids::TOUCHPAD_ACTION
    }
}

// Allow Into TouchAction from message
impl Into<TouchAction> for super::Message {
    fn into(self) -> TouchAction {
        new(self.get_payload_bytes())
    }
}
