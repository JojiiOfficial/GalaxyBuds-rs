use super::Message;

#[derive(Debug)]
pub struct Simple {
    pub data: u8,
    msg_id: u8,
    response: bool,
}

pub fn new(msg_id: u8, data: u8) -> Simple {
    Simple {
        msg_id,
        data,
        response: false,
    }
}

pub fn new_response(msg_id: u8, data: u8) -> Simple {
    Simple {
        msg_id,
        data,
        response: true,
    }
}

impl Message for Simple {
    fn get_data(&self) -> Vec<u8> {
        vec![self.data]
    }

    fn get_id(&self) -> u8 {
        self.msg_id
    }

    fn is_response(&self) -> bool {
        self.response
    }
}
