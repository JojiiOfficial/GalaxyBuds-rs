#![allow(dead_code)]
use galaxy_buds_rs::{
    message::{self, ids, Message},
    model::Model,
};

use async_std::io::prelude::*;
use bluetooth_serial_port_async::{BtAddr, BtProtocol, BtSocket};
use std::{env, error::Error, str::FromStr};

async fn run() -> Result<(), Box<dyn Error>> {
    let address = env::args().nth(1).unwrap();

    let mut socket = BtSocket::new(BtProtocol::RFCOMM).unwrap();
    socket
        .connect(BtAddr::from_str(address.as_ref()).unwrap())
        .unwrap();

    // Get the stream of the socket. Only call this function
    // once and keep using the stream
    let mut stream = socket.get_stream();

    let mut buffer = [0; 2048];
    loop {
        let num_bytes_read = stream.read(&mut buffer[..]).await.unwrap();
        let buff = &buffer[0..num_bytes_read];

        let id = buff[3].to_be();
        let message = Message::new(buff, Model::Buds);
        println!("{:?}", buff);

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
