use super::{
    bud_property::{BudProperty, EqualizerType},
    ids, simple,
};

pub fn new_equalizer(d: EqualizerType) -> simple::Simple {
    simple::new(ids::EQUALIZER, d.encode())
}
