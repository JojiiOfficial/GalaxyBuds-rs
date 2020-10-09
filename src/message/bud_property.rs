use crate::utils::byteutil;

pub trait BudProperty {
    type Item;

    fn side_val(val: u8, side: Side) -> u8 {
        match side {
            Side::Left => byteutil::value_of_left(val),
            Side::Right => byteutil::value_of_right(val),
        }
    }

    fn value(val: u8, side: Side) -> Self::Item {
        Self::decode(Self::side_val(val, side))
    }

    fn decode(val: u8) -> Self::Item;

    fn encode(&self) -> u8 {
        0
    }
}

#[derive(Debug, PartialEq)]
pub enum Side {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Placement {
    InOpenCase,
    Outside,
    Ear,
    InCloseCase,
    Undetected,
}

impl BudProperty for Placement {
    type Item = Placement;

    fn decode(val: u8) -> Placement {
        match val {
            1 => Placement::Ear,
            2 => Placement::Outside,
            3 => Placement::InOpenCase,
            4 => Placement::InCloseCase,
            _ => Placement::Undetected,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TouchpadOption {
    NoiseCanceling,
    VoiceCommand,
    Volume,
    Spotify,
    Undetected,
}

impl BudProperty for TouchpadOption {
    type Item = TouchpadOption;

    fn decode(val: u8) -> TouchpadOption {
        match val {
            1 => TouchpadOption::VoiceCommand,
            2 => TouchpadOption::NoiseCanceling,
            3 => TouchpadOption::Volume,
            4 => TouchpadOption::Spotify,
            _ => TouchpadOption::Undetected,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum EqualizerType {
    Normal,
    BassBoost,
    Soft,
    Dynamic,
    Clear,
    TrebleBoost,
    Undetected,
}

impl BudProperty for EqualizerType {
    type Item = EqualizerType;

    fn decode(val: u8) -> EqualizerType {
        match val {
            0 => EqualizerType::Normal,
            1 => EqualizerType::BassBoost,
            2 => EqualizerType::Soft,
            3 => EqualizerType::Dynamic,
            4 => EqualizerType::Clear,
            5 => EqualizerType::TrebleBoost,
            _ => EqualizerType::Undetected,
        }
    }

    fn encode(&self) -> u8 {
        match *self {
            EqualizerType::Normal => 0,
            EqualizerType::BassBoost => 2,
            EqualizerType::Soft => 2,
            EqualizerType::Dynamic => 3,
            EqualizerType::Clear => 4,
            EqualizerType::TrebleBoost => 5,
            EqualizerType::Undetected => 10,
        }
    }
}
