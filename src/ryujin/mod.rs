use crate::usb::UsbDevice;

mod payload;
mod ryujin_driver;

const VID: u16 = 0x0b05;
const PID: u16 = 0x1aa2;
const CONTROL_ENDPOINT_ADDRESS: u8 = 0x02;

#[derive(Debug)]
pub struct RyujinDriver {
    usb_device: UsbDevice,
}

#[derive(Clone)]
struct Payload {
    inner: Vec<u8>,
}

#[derive(Clone, Debug)]
pub enum DefaultAnimation {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
}
