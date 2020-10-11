// Message ID: 145 (-111)
use super::{ids, Msg};

#[derive(Debug)]
pub struct TouchUpdated {
    pub status: bool,
}

pub fn new(arr: &[u8]) -> TouchUpdated {
    TouchUpdated {
        status: arr[0] == 1,
    }
}

impl Msg for TouchUpdated {
    fn get_id(&self) -> u8 {
        ids::TOUCH_UPDATED
    }
}
