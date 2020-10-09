use super::{ids, Message};

#[derive(Debug)]
pub struct LockTouchpad {
    pub lock: bool,
}

pub fn new(lock: bool) -> LockTouchpad {
    LockTouchpad { lock }
}

impl Message for LockTouchpad {
    fn get_data(&self) -> Vec<u8> {
        vec![self.lock.into()]
    }

    fn get_id(&self) -> u8 {
        ids::LOCK_TOUCHPAD
    }
}
