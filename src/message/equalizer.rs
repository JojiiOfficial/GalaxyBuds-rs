use super::{
    bud_property::{BudProperty, EqualizerType},
    ids, simple,
};

pub fn new(d: EqualizerType) -> simple::Simple {
    simple::new(ids::EQUALIZER, d.encode())
}
