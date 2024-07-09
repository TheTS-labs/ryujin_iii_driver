use super::FanPreset;

impl FanPreset {
    pub fn into_command(self) -> [u8; 2] {
        match self {
            FanPreset::FullSpeed => [0x64, 0x64],
            FanPreset::Turbo => [0x37, 0x46],
            FanPreset::Standard => [0x28, 0x32],
            FanPreset::Silent => [0x14, 0x1e],
        }
    }
}
