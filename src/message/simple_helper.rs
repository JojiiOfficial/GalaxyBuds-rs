use super::{
    bud_property::{BudProperty, EqualizerType},
    ids, simple,
};

pub fn new_equalizer(d: EqualizerType) -> simple::Simple {
    simple::new(ids::EQUALIZER, d.encode())
}

pub fn new_adjust_sound_sync(adjust: bool) -> simple::Simple {
    simple::new(ids::ADJUST_SOUND_SYNC, adjust.into())
}
