# GalaxyBuds-rs
[![crates](https://img.shields.io/crates/dv/galaxy_buds_rs?style=flat-square)](https://crates.io/crates/galaxy_buds_rs)
![PRs](https://img.shields.io/badge/PRs-welcome-56cc14?style=flat-square)

A reverse engineered rust wrapper for the GalaxyBuds bluetooth protocol. Can be used to communicate with your earbuds using rust.
You can find a cli tool controlling your Earbuds on linux [here](https://github.com/JojiiOfficial/LiveBudsCli)

#### To use:
Add this to your Cargo.toml
```
galaxy_buds_rs = "0.2.1"
```
Or if you have `cargo edit`:
```
cargo add galaxy_buds_rs
```

# Features

### Receiving
- [x] Status update
- [x] Extended status update
- [x] Get all debug data
- [x] Touch updated
- [x] Voice wakeup listening update
- [x] Touchpad tap action
- [ ] Version info

### Sending
- [x] Un/Lock touchpad
- [x] Set noisereduction
- [x] Set Equalizer
- [x] Adjust sound sync
- [x] Mute earbud
- [x] Find my earbuds
- [x] Prepare voice notification (notifications TTS)
- [x] Set touchpad option
- [ ] Update time

# Examples

### Receive
Set the `address` value in `examples/receive.rs` to your Buds' mac address and run following:
```bash
cargo --example receive
```

### Send
Set the `address` value in `examples/send.rs` to your Buds' mac address and run following:
```bash
cargo --example send
```
