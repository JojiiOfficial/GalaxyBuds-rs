use super::{
    bud_property::{BudProperty, TouchpadOption},
    ids, Payload,
};

/// Lock or unlock the touchpad
#[derive(Debug)]
pub struct SetTouchpadOption {
    left_option: TouchpadOption,
    right_option: TouchpadOption,
}

pub fn new(left_option: TouchpadOption, right_option: TouchpadOption) -> SetTouchpadOption {
    SetTouchpadOption {
        left_option,
        right_option,
    }
}

impl Payload for SetTouchpadOption {
    fn get_data(&self) -> Vec<u8> {
        vec![self.left_option.encode(), self.right_option.encode()]
    }

    fn get_id(&self) -> u8 {
        ids::SET_TOUCHPAD_OPTION
    }
}
