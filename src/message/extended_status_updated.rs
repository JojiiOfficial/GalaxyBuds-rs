use super::bytebuff::ByteBuff;

use super::bud_property::{BudProperty, EqualizerType, Placement, Side, TouchpadOption};
use super::{ids, Payload};

const DEVICE_COLOR_BLACK: u8 = 2;
const DEVICE_COLOR_PINK: u8 = 4;
const DEVICE_COLOR_WHITE: u8 = 0;
const DEVICE_COLOR_YELLOW: u8 = 3;
const TYPE_KERNEL: u8 = 0;
const TYPE_OPEN: u8 = 1;

#[derive(Debug)]
pub struct ExtendedStatusUpdate {
    pub revision: u8,
    pub ear_type: u8,
    pub battery_left: i8,
    pub battery_right: i8,
    pub coupled: bool,
    pub primary_earbud: Side,
    pub placement_left: Placement,
    pub placement_right: Placement,
    pub wearing_left: bool,
    pub wearing_right: bool,
    pub battery_case: i8,
    pub adjust_sound_sync: bool,
    pub equalizer_type: EqualizerType,
    pub touchpads_blocked: bool,
    pub touchpad_option_left: TouchpadOption,
    pub touchpad_option_right: TouchpadOption,
    pub noise_reduction: bool,
    pub voice_wake_up: bool,
    pub color: i16,
}

pub fn new(arr: &[u8]) -> ExtendedStatusUpdate {
    let buff = ByteBuff::new(&arr);

    let placement_left = Placement::value(buff.get(6), Side::Left);
    let placement_right = Placement::value(buff.get(6), Side::Right);

    ExtendedStatusUpdate {
        revision: buff.get(0),
        ear_type: buff.get(1),
        battery_left: buff.get(2) as i8,
        battery_right: buff.get(3) as i8,
        coupled: buff.get_bool(4),
        primary_earbud: Side::from(buff.get_bool(5)),
        placement_left,
        placement_right,
        wearing_left: placement_left == Placement::Ear,
        wearing_right: placement_right == Placement::Ear,
        battery_case: arr[7] as i8,
        adjust_sound_sync: buff.get_bool(8),
        equalizer_type: EqualizerType::decode(buff.get(9)),
        touchpads_blocked: arr[10] == 1,
        touchpad_option_left: TouchpadOption::value(buff.get(11), Side::Left),
        touchpad_option_right: TouchpadOption::value(buff.get(11), Side::Right),
        noise_reduction: buff.get_bool(12),
        voice_wake_up: buff.get_bool(13),
        color: buff.get_short(14),
    }
}

impl Payload for ExtendedStatusUpdate {
    fn get_id(&self) -> u8 {
        ids::EXTENDED_STATUS_UPDATED
    }
}

/// Allow parsing a Message to an ExtendedStatusUpdate
impl Into<ExtendedStatusUpdate> for super::Message {
    fn into(self) -> ExtendedStatusUpdate {
        new(self.get_payload_bytes())
    }
}
