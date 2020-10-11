use super::{ids, Msg};

/// Enable/Disable ANC for your earbuds
#[derive(Debug)]
pub struct SetNoiseReduction {
    pub noise_reduction: bool,
}

pub fn new(noise_reduction: bool) -> SetNoiseReduction {
    SetNoiseReduction { noise_reduction }
}

impl Msg for SetNoiseReduction {
    fn get_data(&self) -> Vec<u8> {
        vec![self.noise_reduction.into()]
    }

    fn get_id(&self) -> u8 {
        ids::SET_NOISE_REDUCTION
    }
}
