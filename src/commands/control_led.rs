use crate::*;

/// Makes the reader's led glow green `freq` times for `duration`.
pub fn control_led(freq: u8, duration: u8) -> Result<(), ReaderError> {
    let device = CFRH320U93::init()?;
    let mut buffer = Buffer::new();
    buffer.write(0x03);
    buffer.write(0x88);
    buffer.write(duration);
    buffer.write(freq);

    device.set_report(buffer.get())?;

    let read_buf = device.get_report()?;

    if StatusCode::from(read_buf[12]) != StatusCode::Ok {
        return Err(StatusCode::from(read_buf[12]).into());
    }

    Ok(())
}

/// Turns the reader's led green.
pub fn green_led() -> Result<(), ReaderError> {
    control_led(0xff,0xff)
}

/// Turns the reader's led red.
pub fn red_led() -> Result<(), ReaderError> {
    control_led(0x00,0x00)
}
