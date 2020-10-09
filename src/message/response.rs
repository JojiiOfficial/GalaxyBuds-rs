use super::{ids, simple};

pub fn new_status_updated() -> simple::Simple {
    simple::new_response(ids::STATUS_UPDATED, 0)
}

pub fn new_extended_status_updated() -> simple::Simple {
    simple::new_response(ids::EXTENDED_STATUS_UPDATED, 0)
}

pub fn new_version_info() -> simple::Simple {
    simple::new_response(ids::VERSION_INFO, 0)
}

pub fn new_voice_wake_up_event() -> simple::Simple {
    simple::new_response(ids::VOICE_WAKE_UP_EVENT, 0)
}
