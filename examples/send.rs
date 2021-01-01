#![allow(dead_code)]
use galaxy_buds_rs::message::{self, bud_property::EqualizerType, Payload}; // Note: Import 'Payload' to be able to convert the message to bytes

use async_std::io::prelude::*;
use bluetooth_serial_port_async::{BtAddr, BtProtocol, BtSocket}; /* https://crates.io/crates/bluetooth-serial-port-async */
use std::{env, error::Error, str::FromStr};

async fn run() -> Result<(), Box<dyn Error>> {
    let address = env::args().nth(1).unwrap();

    let mut socket = BtSocket::new(BtProtocol::RFCOMM).expect("RFCOMM");
    socket
        .connect(BtAddr::from_str(address.as_ref()).expect("Address"))
        .expect("Socket not connecting");

    // Get the stream of the socket. Only call this function
    // once and keep using the stream
    let mut stream = socket.get_stream();

    // Lock the touchpads
    let send_msg = message::lock_touchpad::new(true);
    stream.write(&send_msg.to_byte_array()).await?;

    // Set the equalizer to 'bass boost'
    let send_msg = message::simple::new_equalizer(EqualizerType::BassBoost);
    stream.write(&send_msg.to_byte_array()).await?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    async_std::task::block_on(run())
}
