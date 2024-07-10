mod ryujin;
mod usb;

use ryujin::RyujinDriver;

fn main() -> rusb::Result<()> {
    let ryujin = RyujinDriver::new()?;

    ryujin.set_duty(100, 0)?;

    Ok(())
}
