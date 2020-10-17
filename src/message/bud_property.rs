use crate::utils::byteutil;

/// A property value of a single earbud
pub trait BudProperty {
    type Item;

    /// Get the corresponding value of a byte based on the
    /// side of the earbud (left/right)
    fn side_val(val: u8, side: Side) -> u8 {
        match side {
            Side::Left => byteutil::value_of_left(val),
            Side::Right => byteutil::value_of_right(val),
        }
    }

    /// Get the property item decoded based on
    /// the side and msg byte
    fn value(val: u8, side: Side) -> Self::Item {
        Self::decode(Self::side_val(val, side))
    }

    /// Decode the value. Returns a property variant
    fn decode(val: u8) -> Self::Item;

    /// Needs to be implemented to send a
    /// property variant inside a msg payload
    fn encode(&self) -> u8;
}

/// The side of an earbud.
#[derive(Debug, PartialEq)]
pub enum Side {
    Left,
    Right,
}

impl From<bool> for Side {
    fn from(inp_side: bool) -> Side {
        if !inp_side {
            Side::Right
        } else {
            Side::Left
        }
    }
}

/// Where an earbud is placed in the
/// physical real life world
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Placement {
    InOpenCase,
    Outside,
    Ear,
    InCloseCase,
    Undetected,
}

/// Placement comes inside some payloads so
/// define the decode() here
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

    fn encode(&self) -> u8 {
        match *self {
            Placement::Ear => 1,
            Placement::Outside => 2,
            Placement::InOpenCase => 3,
            Placement::InCloseCase => 4,
            Placement::Undetected => 0,
        }
    }
}

/// Which option is set for holding the touchpad
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

    fn encode(&self) -> u8 {
        match *self {
            TouchpadOption::VoiceCommand => 1,
            TouchpadOption::NoiseCanceling => 2,
            TouchpadOption::Volume => 3,
            TouchpadOption::Spotify => 4,
            TouchpadOption::Undetected => 0,
        }
    }

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

/// The selected EqualizerType
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
            EqualizerType::BassBoost => 1,
            EqualizerType::Soft => 2,
            EqualizerType::Dynamic => 3,
            EqualizerType::Clear => 4,
            EqualizerType::TrebleBoost => 5,
            EqualizerType::Undetected => 10,
        }
    }
}
