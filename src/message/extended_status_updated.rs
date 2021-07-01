use super::bud_property::{BudProperty, EqualizerType, Placement, Side, TouchpadOption};
use super::{bud_property::AmbientType, bytebuff::ByteBuff};
use super::{ids, Payload};
use crate::model::Model;

pub const DEVICE_COLOR_BLACK: u8 = 2;
pub const DEVICE_COLOR_PINK: u8 = 4;
pub const DEVICE_COLOR_WHITE: u8 = 0;
pub const DEVICE_COLOR_YELLOW: u8 = 3;
pub const TYPE_KERNEL: u8 = 0;
pub const TYPE_OPEN: u8 = 1;

#[derive(Debug, Clone, Copy)]
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
    pub color_left: i16,
    pub color_right: i16,
    pub ambient_sound_enabled: bool,
    pub ambient_sound_volume: i32,
    pub extra_high_ambient: bool,
    pub ambient_mode: AmbientType,
}

pub fn new(arr: &[u8], model: Model) -> ExtendedStatusUpdate {
    let buff = ByteBuff::new(&arr);

    let placement_left = Placement::value(buff.get(6), Side::Left);
    let placement_right = Placement::value(buff.get(6), Side::Right);

    match model {
        Model::BudsLive => ExtendedStatusUpdate {
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
            battery_case: buff.get(7) as i8,
            adjust_sound_sync: buff.get_bool(8),
            equalizer_type: EqualizerType::decode(buff.get(9)),
            touchpads_blocked: buff.get_bool(10),
            touchpad_option_left: TouchpadOption::value(buff.get(11), Side::Left),
            touchpad_option_right: TouchpadOption::value(buff.get(11), Side::Right),
            noise_reduction: buff.get_bool(12),
            voice_wake_up: buff.get_bool(13),
            color_left: buff.get_short(14),
            color_right: buff.get_short(16),
            ambient_sound_volume: 0,
            ambient_sound_enabled: false,
            ambient_mode: AmbientType::Normal,
            extra_high_ambient: false,
        },

        Model::BudsPlus => ExtendedStatusUpdate {
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
            battery_case: buff.get(7) as i8,
            adjust_sound_sync: buff.get_bool(10),
            equalizer_type: EqualizerType::decode(buff.get(11)),
            touchpads_blocked: buff.get_bool(12),
            touchpad_option_left: TouchpadOption::value(buff.get(13), Side::Left),
            touchpad_option_right: TouchpadOption::value(buff.get(13), Side::Right),
            color_left: buff.get_short(15),
            color_right: buff.get_short(17),
            voice_wake_up: false,
            noise_reduction: false,
            ambient_sound_enabled: buff.get_bool(8),
            ambient_sound_volume: buff.get(9) as i32,
            ambient_mode: AmbientType::Normal,
            extra_high_ambient: {
                if buff.get(0) >= 9 {
                    buff.get_bool(19)
                } else {
                    false
                }
            },
        },

        Model::BudsPro => ExtendedStatusUpdate {
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
            battery_case: buff.get(7) as i8,
            adjust_sound_sync: buff.get_bool(8),
            equalizer_type: EqualizerType::decode(buff.get(9)),
            touchpads_blocked: buff.get_bool(10),
            touchpad_option_left: TouchpadOption::value(buff.get(11), Side::Left),
            touchpad_option_right: TouchpadOption::value(buff.get(11), Side::Right),
            noise_reduction: buff.get_bool(12),
            voice_wake_up: buff.get_bool(13),
            color_left: buff.get_short(14),
            color_right: buff.get_short(16),
            ambient_sound_volume: 0,
            ambient_sound_enabled: false,
            ambient_mode: AmbientType::Normal,
            extra_high_ambient: false,
        },

        _ => unimplemented!(),
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
        new(self.get_payload_bytes(), self.model)
    }
}
