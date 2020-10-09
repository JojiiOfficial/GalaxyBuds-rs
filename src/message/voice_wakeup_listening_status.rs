// Message ID: 156 (-100)
#[derive(Debug)]
pub struct VoicewakeUpListeningStatus {
    pub voice_wakeup_listening_status: bool,
}

pub fn new(arr: &[u8]) -> VoicewakeUpListeningStatus {
    VoicewakeUpListeningStatus {
        voice_wakeup_listening_status: arr[0] == 1,
    }
}
