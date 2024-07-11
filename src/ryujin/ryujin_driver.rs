use rusb::Context;

use super::{DefaultAnimation, Payload, RyujinDriver, CONTROL_ENDPOINT_ADDRESS, PID, VID};
use crate::usb::{Endpoint, UsbDevice, UsbInfo};

impl RyujinDriver {
    pub fn new() -> rusb::Result<Self> {
        log::info!(target: "ryujin_driver", "Initializing Ryujin III driver");

        log::debug!(target: "ryujin_driver", "Creating context...");
        let context = Context::new()?;

        log::debug!(target: "ryujin_driver", "Creating UsbDevice...");
        let usb_device = UsbDevice::new(VID, PID, &context)?;

        Ok(Self { usb_device })
    }

    pub fn set_duty(&self, pump_duty: u8, vrm_duty: u8) -> rusb::Result<()> {
        assert!(pump_duty <= 100, "pump duty must be between 0 and 100");
        assert!(vrm_duty <= 100, "vrm duty must be between 0 and 100");

        log::info!(target: "ryujin_driver", "Setting pump duty to {}% and vrm duty to {}%...", pump_duty, vrm_duty);

        let endpoint = self.get_control_endpoint()?;
        let in_use = self.usb_device.setup_endpoint(&endpoint)?;
        let payload = Payload::build(None).add(&[0x1a, 0x01, pump_duty, vrm_duty]).finish();

        in_use.write_interrupt(&payload, None)?;

        in_use.release()?;

        Ok(())
    }

    pub fn turn_off_screen(&self) -> rusb::Result<()> {
        log::info!(target: "ryujin_driver", "Turning off screen...");

        let endpoint = self.get_control_endpoint()?;
        let in_use = self.usb_device.setup_endpoint(&endpoint)?;
        let payload = Payload::build(None).add(&[0x51]).finish();

        in_use.write_interrupt(&payload, None)?;

        in_use.release()?;

        Ok(())
    }

    pub fn print_text_on_screen(&self, text: &str) -> rusb::Result<()> {
        log::info!(target: "ryujin_driver", "Printing text on screen: {}", text);

        let endpoint = self.get_control_endpoint()?;
        let in_use = self.usb_device.setup_endpoint(&endpoint)?;
        let payload = Payload::build(None).add(&[0x53, 0x00]).add(text.as_bytes()).finish();

        in_use.write_interrupt(&payload, None)?;

        in_use.release()?;

        Ok(())
    }

    pub fn set_default_animation(&self, animation: DefaultAnimation) -> rusb::Result<()> {
        log::info!(target: "ryujin_driver", "Setting default animation: {:?}", animation);
        let endpoint = self.get_control_endpoint()?;
        let in_use = self.usb_device.setup_endpoint(&endpoint)?;
        let payload = Payload::build(None).add(&[0x51, 0x14, 0x00, animation as u8]).finish();

        in_use.write_interrupt(&payload, None)?;

        in_use.release()?;

        Ok(())
    }

    pub fn info(&self) -> rusb::Result<UsbInfo> { self.usb_device.info(None) }

    fn get_control_endpoint(&self) -> rusb::Result<Endpoint> {
        log::debug!(target: "ryujin_driver", "Getting control endpoint...");

        self.usb_device
            .readable_endpoints()
            .into_iter()
            .find(|e| e.address == CONTROL_ENDPOINT_ADDRESS)
            .ok_or(rusb::Error::NotFound)
    }
}
