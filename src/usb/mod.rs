mod endpoint_in_use;
mod usb_device;

use rusb::{Context, Device, DeviceDescriptor, DeviceHandle, Language};

#[derive(Clone, Debug)]
pub struct Endpoint {
    pub config:    u8,
    pub interface: u8,
    pub setting:   u8,
    pub address:   u8,
}

#[derive(Clone, Debug)]
pub struct UsbInfo {
    pub language:      Language,
    pub manufacturer:  Option<String>,
    pub product:       Option<String>,
    pub serial_number: Option<String>,
}

#[derive(Debug)]
pub struct UsbDevice {
    pub device:     Device<Context>,
    pub descriptor: DeviceDescriptor,
    pub handle:     DeviceHandle<Context>,
}

pub struct EndpointInUse<'a, 'b> {
    had_kernel_driver: bool,
    usb:               &'a UsbDevice,
    endpoint:          &'b Endpoint,
}
