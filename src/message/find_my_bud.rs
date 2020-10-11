use super::{ids, Payload};

/// Find my bud
/// set start to true and send the payload to start the
/// feature. Set start to false and send the payloaod again
/// to stop it.
#[derive(Debug)]
pub struct FindMyBud {
    pub start: bool,
}

pub fn new(start: bool) -> FindMyBud {
    FindMyBud { start }
}

impl Payload for FindMyBud {
    fn get_data(&self) -> Vec<u8> {
        vec![]
    }

    fn get_id(&self) -> u8 {
        if self.start {
            ids::FIND_MY_EARBUDS_START
        } else {
            ids::FIND_MY_EARBUDS_STOP
        }
    }
}
