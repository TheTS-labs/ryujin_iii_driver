use rusb::Context;

use super::{FanPreset, Payload, RyujinDriver, FAN_CONTROL_ENDPOINT_ADDRESS, PID, VID};
use crate::usb::UsbDevice;

impl RyujinDriver {
    pub fn new() -> rusb::Result<Self> {
        let context = Context::new()?;
        let usb_device = UsbDevice::new(VID, PID, &context)?;

        Ok(Self { usb_device })
    }

    pub fn set_fan_preset(&self, preset: FanPreset) -> rusb::Result<()> {
        let endpoint = self
            .usb_device
            .readable_endpoints()
            .into_iter()
            .find(|e| e.address == FAN_CONTROL_ENDPOINT_ADDRESS)
            .ok_or(rusb::Error::NotFound)?;

        let in_use = self.usb_device.setup_endpoint(&endpoint)?;
        let payload = Payload::build()
            .add(&[0xec, 0x1a, 0x01])
            .add(&preset.into_command())
            .finish();

        in_use.write_interrupt(&payload, None)?;

        in_use.release()?;

        Ok(())
    }
}
