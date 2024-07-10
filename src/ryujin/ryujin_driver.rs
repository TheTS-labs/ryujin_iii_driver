use rusb::Context;

use super::{Payload, RyujinDriver, FAN_CONTROL_ENDPOINT_ADDRESS, PID, VID};
use crate::usb::UsbDevice;

impl RyujinDriver {
    pub fn new() -> rusb::Result<Self> {
        let context = Context::new()?;
        let usb_device = UsbDevice::new(VID, PID, &context)?;

        Ok(Self { usb_device })
    }

    pub fn set_duty(&self, pump_duty: u8, vrm_duty: u8) -> rusb::Result<()> {
        assert!(pump_duty <= 100, "pump duty must be between 0 and 100");
        assert!(vrm_duty <= 100, "vrm duty must be between 0 and 100");

        let endpoint = self
            .usb_device
            .readable_endpoints()
            .into_iter()
            .find(|e| e.address == FAN_CONTROL_ENDPOINT_ADDRESS)
            .ok_or(rusb::Error::NotFound)?;

        let in_use = self.usb_device.setup_endpoint(&endpoint)?;
        let payload = Payload::build().add(&[0xec, 0x1a, 0x01, pump_duty, vrm_duty]).finish();

        in_use.write_interrupt(&payload, None)?;

        in_use.release()?;

        Ok(())
    }

    // pub fn turn_off_screen(&self) -> rusb::Result<()> {
    //     let endpoint = self
    //         .usb_device
    //         .readable_endpoints()
    //         .into_iter()
    //         .find(|e| e.address == FAN_CONTROL_ENDPOINT_ADDRESS)
    //         .ok_or(rusb::Error::NotFound)?;

    //     let in_use = self.usb_device.setup_endpoint(&endpoint)?;
    //     let payload = Payload::build().add(&[0xec, 0x51]).finish();

    //     in_use.write_interrupt(&payload, None)?;

    //     in_use.release()?;

    //     Ok(())
    // }

    // pub fn print_text_on_screen(&self, text: &str) -> rusb::Result<()> {
    //     let endpoint = self
    //         .usb_device
    //         .readable_endpoints()
    //         .into_iter()
    //         .find(|e| e.address == FAN_CONTROL_ENDPOINT_ADDRESS)
    //         .ok_or(rusb::Error::NotFound)?;

    //     let in_use = self.usb_device.setup_endpoint(&endpoint)?;
    //     let payload = Payload::build().add(&[0xec, 0x53, 0x00]).add(text.as_bytes()).finish();

    //     in_use.write_interrupt(&payload, None)?;

    //     in_use.release()?;

    //     Ok(())
    // }
}
