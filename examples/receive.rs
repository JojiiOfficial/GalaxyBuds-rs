#![allow(dead_code)]
use galaxy_buds_live_rs::message::{self, ids, Message};

use async_std::io::prelude::*;
use bluetooth_serial_port_async::{BtAddr, BtProtocol, BtSocket};
use std::{error::Error, str::FromStr};

async fn run() -> Result<(), Box<dyn Error>> {
    let mut socket = BtSocket::new(BtProtocol::RFCOMM).unwrap();

    let address = "<Your Buds address here!!>";
    socket.connect(&BtAddr::from_str(address).unwrap()).unwrap();

    let mut buffer = [0; 2048];
    let mut stream = socket.get_stream();

    loop {
        let num_bytes_read = stream.read(&mut buffer[..]).await.unwrap();
        let buff = &buffer[0..num_bytes_read];

        let id = buff[3].to_be();
        let message = Message::new(buff);

        if id == 242 {
            continue;
        }

        // Print touchpad taps
        if id == ids::TOUCHPAD_ACTION {
            let msg: message::touchpad_action::TouchAction = message.into();
            println!("{:?}", msg);
            continue;
        }

        // Print status updates
        if id == ids::STATUS_UPDATED {
            let msg: message::status_updated::StatusUpdate = message.into();
            println!("{:?}", msg);
            continue;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    async_std::task::block_on(run())
}
