use super::bud_property::{BudProperty, Placement, Side};
use super::{ids, Payload};

#[derive(Debug, Clone, Copy)]
pub struct StatusUpdate {
    pub revision: u8,
    pub battery_left: i8,
    pub battery_right: i8,
    pub coupled: bool,
    pub primary_earbud: u8,
    pub placement_left: Placement,
    pub placement_right: Placement,
    pub wearing_left: bool,
    pub wearing_right: bool,
    pub battery_case: i8,
}

pub fn new(arr: &[u8]) -> StatusUpdate {
    let placement_left = Placement::value(arr[5], Side::Left);
    let placement_right = Placement::value(arr[5], Side::Right);

    StatusUpdate {
        revision: arr[0],
        battery_left: arr[1] as i8,
        battery_right: arr[2] as i8,
        coupled: arr[3] == 1,
        primary_earbud: arr[4],
        placement_left,
        placement_right,
        wearing_left: placement_left == Placement::Ear,
        wearing_right: placement_right == Placement::Ear,
        battery_case: arr[6] as i8,
    }
}

impl Payload for StatusUpdate {
    fn get_id(&self) -> u8 {
        ids::STATUS_UPDATED
    }
}

// Allow parsing Message to a StatusUpdate
impl Into<StatusUpdate> for super::Message {
    fn into(self) -> StatusUpdate {
        new(self.get_payload_bytes())
    }
}
