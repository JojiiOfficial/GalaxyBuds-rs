#![allow(dead_code)]
use galaxy_buds_live_rs::message::{self, Payload}; // Note: Import 'Payload' to be able to convert the message to bytes

use async_std::io::prelude::*;
use bluetooth_serial_port_async::{BtAddr, BtProtocol, BtSocket}; /* https://crates.io/crates/bluetooth-serial-port-async */
use std::{error::Error, str::FromStr, thread::sleep, time::Duration};

async fn run() -> Result<(), Box<dyn Error>> {
    let address = "<Your Buds address here!!>";

    let mut socket = BtSocket::new(BtProtocol::RFCOMM).unwrap();
    socket.connect(&BtAddr::from_str(address).unwrap()).unwrap();

    // Get the stream of the socket. Only call this function
    // once and keep using the stream
    let mut stream = socket.get_stream();

    let mut find = message::find_my_bud::new(true);

    // Start making noise
    stream.write(&find.to_byte_array()).await?;
    sleep(Duration::from_secs(3));

    // mute left
    let mute = message::mute_earbud::new(true, false);
    stream.write(&mute.to_byte_array()).await?;
    sleep(Duration::from_secs(3));

    // mute right
    let mute = message::mute_earbud::new(false, true);
    stream.write(&mute.to_byte_array()).await?;
    sleep(Duration::from_secs(3));

    // stop
    find.start = false;
    stream.write(&find.to_byte_array()).await?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    async_std::task::block_on(run())
}
