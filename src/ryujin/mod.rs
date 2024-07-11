use crate::usb::UsbDevice;

mod payload;
mod ryujin_driver;

const VID: u16 = 0x0b05;
const PID: u16 = 0x1aa2;
const CONTROL_ENDPOINT_ADDRESS: u8 = 0x02;

pub struct RyujinDriver {
    usb_device: UsbDevice,
}

pub struct Payload {
    inner: Vec<u8>,
}