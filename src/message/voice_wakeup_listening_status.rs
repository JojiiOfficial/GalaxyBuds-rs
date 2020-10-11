use super::{ids, Msg};

#[derive(Debug)]
pub struct VoicewakeUpListeningStatus {
    pub voice_wakeup_listening_status: bool,
}

pub fn new(arr: &[u8]) -> VoicewakeUpListeningStatus {
    VoicewakeUpListeningStatus {
        voice_wakeup_listening_status: arr[0] == 1,
    }
}

impl Msg for VoicewakeUpListeningStatus {
    fn get_id(&self) -> u8 {
        ids::VOICE_WAKE_UP_LISTENING_STATUS
    }
}
