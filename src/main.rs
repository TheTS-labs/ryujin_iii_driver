mod ryujin;
mod usb;

use ryujin::RyujinDriver;

fn main() -> rusb::Result<()> {
    let ryujin = RyujinDriver::new()?;

    // ryujin.set_duty(100, 0)?;
    // ryujin.turn_off_screen()?;
    ryujin.set_default_animation(0)?;

    Ok(())
}
