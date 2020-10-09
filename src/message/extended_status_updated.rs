// Message ID: 97

use super::bud_property::{BudProperty, EqualizerType, Placement, Side, TouchpadOption};

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
    pub primary_earbud: u8,
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
}

pub fn new(arr: &[u8]) -> ExtendedStatusUpdate {
    let placement_left = Placement::value(arr[6], Side::Left);
    let placement_right = Placement::value(arr[6], Side::Right);

    ExtendedStatusUpdate {
        revision: arr[0],
        ear_type: arr[1],
        battery_left: arr[2] as i8,
        battery_right: arr[3] as i8,
        coupled: arr[4] == 1,
        primary_earbud: arr[5],
        placement_left,
        placement_right,
        wearing_left: placement_left == Placement::Ear,
        wearing_right: placement_right == Placement::Ear,
        battery_case: arr[7] as i8,
        adjust_sound_sync: arr[8] == 1,
        equalizer_type: EqualizerType::decode(arr[9]),
        touchpads_blocked: arr[10] == 1,
        touchpad_option_left: TouchpadOption::value(arr[11], Side::Left),
        touchpad_option_right: TouchpadOption::value(arr[11], Side::Right),
        noise_reduction: arr[12] == 1,
        voice_wake_up: arr[13] == 1,
    }
}
