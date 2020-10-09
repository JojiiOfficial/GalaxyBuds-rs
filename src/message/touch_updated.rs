// Message ID: 145 (-111)

#[derive(Debug)]
pub struct TouchUpdated {
    pub status: bool,
}

pub fn new(arr: &[u8]) -> TouchUpdated {
    TouchUpdated {
        status: arr[0] == 1,
    }
}
