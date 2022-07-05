use super::{extended_status_updated::ExtTapLockStatus, ids, Payload};

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

#[derive(Debug, Clone, Copy)]
pub struct ExtLockTouchpad {
    // Plax next track
    pub double_tap: bool,
    // Play/Pause track
    pub tap_on: bool,
    // Custom action
    pub touch_and_hold: bool,
    pub touch_controls: bool,
    // Previous track
    pub tripple_tap: bool,
}

impl ExtLockTouchpad {
    pub fn from_ext_tap_lock_status(st: ExtTapLockStatus) -> Self {
        Self {
            double_tap: st.double_tap_on,
            tap_on: st.tap_on,
            touch_and_hold: st.touch_an_hold_on,
            touch_controls: st.touch_controls_on,
            tripple_tap: st.triple_tap_on,
        }
    }

    pub fn new(
        double_tap: bool,
        tap_on: bool,
        touch_and_hold: bool,
        touch_controls: bool,
        tripple_tap: bool,
    ) -> Self {
        Self {
            double_tap,
            tap_on,
            touch_and_hold,
            touch_controls,
            tripple_tap,
        }
    }
}

impl Payload for ExtLockTouchpad {
    fn get_id(&self) -> u8 {
        ids::LOCK_TOUCHPAD
    }

    fn get_data(&self) -> Vec<u8> {
        vec![
            self.touch_controls as u8,
            self.tap_on as u8,
            self.double_tap as u8,
            self.tripple_tap as u8,
            self.touch_and_hold as u8,
            0,
            0,
        ]
    }
}
