/// The device model.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Model {
    Buds,
    BudsPlus,
    BudsLive,
    BudsPro,
}

/// Features which are only available by certain models.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Feature {
    Anc,
    AmbientSound,
    ExtraHighAmbientVolume,
    AmbientVoiceFocus,
    BatteryType,
    OutsideDoubleTap,
    RelieveAmbient,
    Sidetone,
    VoiceWakeup,
    AdjustSoundSync,
}

impl Model {
    /// Returns all available features for the given model.
    pub fn get_features(&self) -> Vec<Feature> {
        match self {
            Model::Buds => {
                vec![
                    Feature::BatteryType,
                    Feature::AmbientSound,
                    Feature::AmbientVoiceFocus,
                ]
            }

            Model::BudsPlus => {
                vec![
                    Feature::AmbientSound,
                    Feature::OutsideDoubleTap,
                    Feature::Sidetone,
                    Feature::ExtraHighAmbientVolume,
                    Feature::AdjustSoundSync,
                ]
            }

            Model::BudsLive => {
                vec![
                    Feature::Anc,
                    Feature::RelieveAmbient,
                    Feature::VoiceWakeup,
                    Feature::AdjustSoundSync,
                ]
            }

            Model::BudsPro => {
                vec![
                    Feature::Anc,
                    Feature::RelieveAmbient,
                    Feature::VoiceWakeup,
                    Feature::AdjustSoundSync,
                ]
            }
        }
    }

    /// Returns true whether a model has the given feature.
    pub fn has_feature(&self, feature: Feature) -> bool {
        self.get_features().iter().any(|i| *i == feature)
    }
}
