use crate::*;

pub fn control_buzzer(freq: u8, duration: u8) -> Result<(), ReaderError> {
    let device = CFRH320U93::init()?;
    let mut buffer = Buffer::new();
    buffer.write(0x03);
    buffer.write(0x89);
    buffer.write(duration);
    buffer.write(freq);

    device.set_report(buffer.get())?; 

    let read_buf = device.get_report()?;

    if StatusCode::from(read_buf[12]) != StatusCode::Ok {
        return Err(StatusCode::from(read_buf[12]).into());
    }

    Ok(())
}