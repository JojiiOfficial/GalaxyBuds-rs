use super::{ids, Payload};

/// Enable/Disable ANC for your earbuds
#[derive(Debug, Clone, Copy)]
pub struct SetManagerInfo {
    pub client_type: u8, // WearableApp = 1, others unknow
    pub is_samsung_device: bool,
    pub android_sdk: u8,
}

pub fn new(is_samsung_device: bool, android_sdk: u8) -> SetManagerInfo {
    SetManagerInfo {
        client_type: 1,
        is_samsung_device,
        android_sdk,
    }
}

impl Payload for SetManagerInfo {
    fn get_data(&self) -> Vec<u8> {
        vec![
            self.client_type,
            {
                if self.is_samsung_device {
                    1
                } else {
                    2
                }
            },
            self.android_sdk,
        ]
    }

    fn get_id(&self) -> u8 {
        ids::MANAGER_INFO
    }
}
