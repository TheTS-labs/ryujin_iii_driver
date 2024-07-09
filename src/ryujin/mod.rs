use crate::usb::UsbDevice;

mod fan_preset;
mod payload;
mod ryujin_driver;

const VID: u16 = 0x0b05;
const PID: u16 = 0x1aa2;
const FAN_CONTROL_ENDPOINT_ADDRESS: u8 = 0x02;

pub struct RyujinDriver {
    usb_device: UsbDevice,
}

pub enum FanPreset {
    FullSpeed,
    Turbo,
    Standard,
    Silent,
}

pub struct Payload {
    inner: Vec<u8>,
}
