use super::Message;

#[derive(Debug)]
pub struct Simple {
    pub data: u8,
    msg_id: u8,
}

pub fn new(msg_id: u8, data: u8) -> Simple {
    Simple { msg_id, data }
}

impl Message for Simple {
    fn get_data(&self) -> Vec<u8> {
        vec![self.data]
    }

    fn get_id(&self) -> u8 {
        self.msg_id
    }
}
