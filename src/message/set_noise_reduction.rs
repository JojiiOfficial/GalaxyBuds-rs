use super::Message;

#[derive(Debug)]
pub struct SetNoiseReduction {
    pub noise_reduction: bool,
}

pub fn new(noise_reduction: bool) -> SetNoiseReduction {
    SetNoiseReduction { noise_reduction }
}

impl Message for SetNoiseReduction {
    fn get_data(&self) -> Vec<u8> {
        vec![self.noise_reduction.into()]
    }

    fn get_id() -> u8 {
        152 // -104 in java
    }
}
