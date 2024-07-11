use std::time::Duration;

use super::{Endpoint, EndpointInUse, UsbDevice};

impl EndpointInUse<'_, '_> {
    pub fn claim_and_config<'a, 'b>(usb: &'a UsbDevice, endpoint: &'b Endpoint) -> rusb::Result<EndpointInUse<'a, 'b>> {
        log::info!(
            target: "ryujin_driver",
            "Claiming and configuring 0x{:02x} endpoint on {:04x}:{:04x}...",
            endpoint.address, usb.descriptor.vendor_id(), usb.descriptor.product_id()
        );

        let kernel_driver_active = usb.handle.kernel_driver_active(endpoint.interface).unwrap_or(false);

        if kernel_driver_active {
            log::debug!(target: "ryujin_driver", "Kernel driver active, detaching...");
            usb.handle.detach_kernel_driver(endpoint.interface)?;
        }

        usb.handle.set_active_configuration(endpoint.config)?;
        usb.handle.claim_interface(endpoint.interface)?;
        usb.handle.set_alternate_setting(endpoint.interface, endpoint.setting)?;

        Ok(EndpointInUse {
            had_kernel_driver: kernel_driver_active,
            usb,
            endpoint,
        })
    }

    pub fn release(self) -> rusb::Result<()> {
        log::info!(
            target: "ryujin_driver",
            "Releasing 0x{:02x} endpoint on {:04x}:{:04x}...",
            self.endpoint.address, self.usb.descriptor.vendor_id(), self.usb.descriptor.product_id()
        );

        self.usb.handle.release_interface(self.endpoint.interface)?;

        if self.had_kernel_driver {
            log::debug!(target: "ryujin_driver", "Kernel driver was active, reattaching...");
            self.usb.handle.attach_kernel_driver(self.endpoint.interface)?;
        }

        Ok(())
    }

    pub fn write_interrupt(&self, data: &[u8], timeout: Option<Duration>) -> rusb::Result<usize> {
        let timeout = timeout.unwrap_or(Duration::from_secs(1));

        log::debug!(
                target: "ryujin_driver",
                "Writing {:02X?} to 0x{:02x} endpoint on {:04x}:{:04x}...",
                data, self.endpoint.address, self.usb.descriptor.vendor_id(), self.usb.descriptor.product_id()
        );

        self.usb.handle.write_interrupt(self.endpoint.address, data, timeout)
    }
}
