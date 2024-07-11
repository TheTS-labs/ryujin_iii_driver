use std::fmt::Debug;

use super::Payload;

impl Debug for Payload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "Payload {:02X?}", self.inner) }
}

impl Payload {
    pub fn build(report_id: Option<u8>) -> Self {
        let mut inner = Vec::with_capacity(65);
        inner.push(report_id.unwrap_or(0xec));

        Self { inner }
    }

    pub fn add(mut self, data: &[u8]) -> Self {
        self.inner.extend_from_slice(data);

        self
    }

    pub fn finish(mut self) -> Vec<u8> {
        if self.inner.len() > 65 {
            panic!("Payload is too big");
        }

        self.inner.resize_with(65, || 0x00);

        self.inner
    }
}
