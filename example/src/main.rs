use ryujin_iii_driver::RyujinDriver;

fn main() -> rusb::Result<()> {
    colog::init();
    let ryujin = RyujinDriver::new()?;

    ryujin.set_duty(100, 0)?;
    ryujin.turn_off_screen()?;

    Ok(())
}
