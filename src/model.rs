use std::fmt::Display;

/// The device model.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Model {
    Buds,
    BudsPlus,
    BudsLive,
    BudsPro,
    BudsPro2,
    Buds2,
    Buds3Pro,
}

impl Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.full_name())
    }
}

/// Features which are only available by certain models.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
    // Extended Touchpad lock
    ExtTouchpadLock,
}

impl Model {
    /// Returns all available features for the given model.
    pub fn get_features(&self) -> &'static [Feature] {
        match self {
            Model::Buds => &[
                Feature::BatteryType,
                Feature::AmbientSound,
                Feature::AmbientVoiceFocus,
            ],
            Model::BudsPlus => &[
                Feature::AmbientSound,
                Feature::OutsideDoubleTap,
                Feature::Sidetone,
                Feature::ExtraHighAmbientVolume,
                Feature::AdjustSoundSync,
            ],
            Model::BudsLive => &[
                Feature::Anc,
                Feature::RelieveAmbient,
                Feature::VoiceWakeup,
                Feature::AdjustSoundSync,
            ],
            Model::BudsPro => &[Feature::Anc, Feature::VoiceWakeup, Feature::AdjustSoundSync],
            Model::BudsPro2 => &[Feature::Anc, Feature::VoiceWakeup, Feature::AdjustSoundSync],
            Model::Buds2 => &[
                Feature::Anc,
                Feature::AmbientSound,
                Feature::OutsideDoubleTap,
                Feature::AdjustSoundSync,
                Feature::ExtTouchpadLock,
            ],
            Model::Buds3Pro => {
                // TODO
                &[Feature::Anc]
            }
        }
    }

    pub fn full_name(&self) -> &'static str {
        match *self {
            Model::Buds => "Galaxy Buds",
            Model::BudsPlus => "Galaxy Buds+",
            Model::BudsLive => "Galaxy Buds Live",
            Model::BudsPro => "Galaxy Buds Pro",
            Model::BudsPro2 => "Galaxy Buds Pro 2",
            Model::Buds2 => "Galaxy Buds 2",
            Model::Buds3Pro => "Galaxy Buds3 Pro",
        }
    }

    /// Returns true whether a model has the given feature.
    pub fn has_feature(&self, feature: &Feature) -> bool {
        self.get_features().contains(feature)
    }
}
