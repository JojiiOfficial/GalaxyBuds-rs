use super::{ids, Payload};

#[derive(Debug, Clone, Copy)]
pub struct VoicewakeUpListeningStatus {
    pub voice_wakeup_listening_status: bool,
}

impl VoicewakeUpListeningStatus {
    pub fn new(arr: &[u8]) -> VoicewakeUpListeningStatus {
        VoicewakeUpListeningStatus {
            voice_wakeup_listening_status: arr[0] == 1,
        }
    }
}
impl Payload for VoicewakeUpListeningStatus {
    fn get_id(&self) -> u8 {
        ids::VOICE_WAKE_UP_LISTENING_STATUS
    }
}

impl From<super::Message> for VoicewakeUpListeningStatus {
    fn from(value: super::Message) -> Self {
        VoicewakeUpListeningStatus::new(value.get_payload_bytes())
    }
}
