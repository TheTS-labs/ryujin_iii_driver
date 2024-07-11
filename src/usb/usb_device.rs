use std::time::Duration;

use rusb::{Context, UsbContext};

use super::{Endpoint, EndpointInUse, UsbDevice, UsbInfo};

impl UsbDevice {
    pub fn new(vendor_id: u16, product_id: u16, context: &Context) -> rusb::Result<Self> {
        log::info!(target: "ryujin_driver", "Initializing USB device {:04x}:{:04x}", vendor_id, product_id);

        let (device, descriptor) = context
            .devices()?
            .iter()
            .filter_map(|device| Some((device.clone(), device.device_descriptor().ok()?)))
            .find(|descriptor| descriptor.1.vendor_id() == vendor_id && descriptor.1.product_id() == product_id)
            .ok_or(rusb::Error::NoDevice)?;

        Ok(Self {
            device: device.clone(),
            descriptor,
            handle: device.open()?,
        })
    }

    pub fn info(&self, timeout: Option<Duration>) -> rusb::Result<UsbInfo> {
        let timeout = timeout.unwrap_or(Duration::from_secs(1));
        let language = self.handle.read_languages(timeout)?[0];

        log::debug!(target: "ryujin_driver", "Getting device info: TIMEOUT: {:?}; LANGUAGE: {:?}", timeout, language);

        Ok(UsbInfo {
            language,
            manufacturer: self
                .handle
                .read_manufacturer_string(language, &self.descriptor, timeout)
                .ok(),
            product: self
                .handle
                .read_product_string(language, &self.descriptor, timeout)
                .ok(),
            serial_number: self
                .handle
                .read_serial_number_string(language, &self.descriptor, timeout)
                .ok(),
        })
    }

    pub fn readable_endpoints(&self) -> Vec<Endpoint> {
        log::debug!(target: "ryujin_driver", "Getting readable endpoints...");
        // TODO: Refactor
        let mut endpoints = vec![];

        let config_descriptors =
            (0..self.descriptor.num_configurations()).filter_map(|n| self.device.config_descriptor(n).ok());

        for config_desc in config_descriptors {
            for interface in config_desc.interfaces() {
                for interface_desc in interface.descriptors() {
                    for endpoint_desc in interface_desc.endpoint_descriptors() {
                        endpoints.push(Endpoint {
                            config:    config_desc.number(),
                            interface: interface_desc.interface_number(),
                            setting:   interface_desc.setting_number(),
                            address:   endpoint_desc.address(),
                        });
                    }
                }
            }
        }

        endpoints
    }

    pub fn setup_endpoint<'a, 'b>(&'a self, endpoint: &'b Endpoint) -> rusb::Result<EndpointInUse<'a, 'b>> {
        EndpointInUse::claim_and_config(self, endpoint)
    }
}
