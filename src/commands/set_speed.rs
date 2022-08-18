use crate::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Speed {
    S9600 = 0x00,
    S19200 = 0x01,
    S38400 = 0x02,
    S57600 = 0x03,
    S115200 = 0x04
}
pub fn set_speed(speed: Speed) ->  Result<(), ReaderError> {
    let device = CFRH320U93::init()?;
    let mut buffer = Buffer::new();
    buffer.write(0x02);
    buffer.write(0x81);
    buffer.write(speed as u8);

    device.set_report(buffer.get())?;

    Ok(())
}