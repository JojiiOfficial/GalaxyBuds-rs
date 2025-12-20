use super::bud_property::{BudProperty, NoiseControlMode, Placement, Side};
use super::{ids, Payload};

// Whether the buds changed the noise control status by a touchpad event
#[derive(Debug, Clone, Copy)]
pub struct NoiseControlsUpdated {
    pub noise_control_mode: NoiseControlMode,
    pub placement_left: Placement,
    pub placement_right: Placement,
}

impl NoiseControlsUpdated {
    pub fn new(arr: &[u8]) -> Self {
        let placement_left = Placement::value(arr[1], Side::Left);
        let placement_right = Placement::value(arr[1], Side::Right);

        Self {
            noise_control_mode: NoiseControlMode::decode(arr[0]),
            placement_left,
            placement_right,
        }
    }
}

impl Payload for NoiseControlsUpdated {
    fn get_id(&self) -> u8 {
        ids::NOISE_CONTROLS_UPDATE
    }
}

// Allow parsing Message to a StatusUpdate
impl Into<NoiseControlsUpdated> for super::Message {
    fn into(self) -> NoiseControlsUpdated {
        NoiseControlsUpdated::new(self.get_payload_bytes())
    }
}
