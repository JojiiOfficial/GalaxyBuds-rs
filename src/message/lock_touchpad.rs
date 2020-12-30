use super::{ids, Payload};

/// Lock or unlock the touchpad
#[derive(Debug, Clone, Copy)]
pub struct LockTouchpad {
    pub lock: bool,
}

pub fn new(lock: bool) -> LockTouchpad {
    LockTouchpad { lock }
}

impl Payload for LockTouchpad {
    fn get_data(&self) -> Vec<u8> {
        vec![self.lock.into()]
    }

    fn get_id(&self) -> u8 {
        ids::LOCK_TOUCHPAD
    }
}
