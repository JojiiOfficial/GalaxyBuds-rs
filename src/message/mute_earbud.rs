use super::{ids, Payload};

// Only available in FindMyGear mode
#[derive(Debug)]
pub struct MuteEarbud {
    pub left_muted: bool,
    pub right_muted: bool,
}

pub fn new(left_muted: bool, right_muted: bool) -> MuteEarbud {
    MuteEarbud {
        left_muted,
        right_muted,
    }
}

impl Payload for MuteEarbud {
    fn get_data(&self) -> Vec<u8> {
        vec![self.left_muted.into(), self.right_muted.into()]
    }

    fn get_id(&self) -> u8 {
        ids::MUTE_EARBUD
    }
}
