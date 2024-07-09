mod ryujin;
mod usb;

use ryujin::{FanPreset, RyujinDriver};

fn main() -> rusb::Result<()> {
    let ryujin = RyujinDriver::new()?;
    ryujin.set_fan_preset(FanPreset::Silent)?;

    Ok(())
}
