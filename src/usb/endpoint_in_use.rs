use std::time::Duration;

use super::{Endpoint, EndpointInUse, UsbDevice};

impl EndpointInUse<'_, '_> {
    pub fn claim_and_config<'a, 'b>(usb: &'a UsbDevice, endpoint: &'b Endpoint) -> rusb::Result<EndpointInUse<'a, 'b>> {
        let kernel_driver_active = usb.handle.kernel_driver_active(endpoint.interface).unwrap_or(false);

        if kernel_driver_active {
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
        self.usb.handle.release_interface(self.endpoint.interface)?;

        if self.had_kernel_driver {
            self.usb.handle.attach_kernel_driver(self.endpoint.interface)?;
        }

        Ok(())
    }

    pub fn write_interrupt(&self, data: &[u8], timeout: Option<Duration>) -> rusb::Result<usize> {
        let timeout = timeout.unwrap_or(Duration::from_secs(1));

        self.usb.handle.write_interrupt(self.endpoint.address, data, timeout)
    }
}
