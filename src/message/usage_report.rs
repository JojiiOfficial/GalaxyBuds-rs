use crate::{message::bytebuff::ByteBuff, utils::byteutil};
use std::collections::HashMap;

use super::{ids, Payload};

#[derive(Debug)]
pub struct UsageReport {
    data: HashMap<String, u32>,
}

impl UsageReport {
    pub fn new(buf: &[u8]) -> Option<Self> {
        let buff = ByteBuff::new(&buf);
        let len = byteutil::to_u8(buff.get(0)) as usize;

        if buff.len() - 1 != len * 9 {
            return None;
        }

        let mut data: HashMap<String, u32> = HashMap::new();

        for i in 0..len {
            let pos = i * 9 + 1;
            let s_sub = buff.range(pos, 5);
            let key = String::from_utf8(s_sub.to_vec()).unwrap();
            let val = buff.get_int(pos + 5);
            data.insert(key, val);
        }

        Some(UsageReport { data })
    }
}

pub struct UsageReportSend;

impl Payload for UsageReportSend {
    fn get_id(&self) -> u8 {
        ids::USAGE_REPORT
    }

    fn get_data(&self) -> Vec<u8> {
        vec![0]
    }
}
