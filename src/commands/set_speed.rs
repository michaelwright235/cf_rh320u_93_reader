use crate::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Speed {
    S9600,
    S19200,
    S38400,
    S57600,
    S115200
}
pub fn set_speed(speed: Speed) ->  Result<(), ReaderError> {
    let device = CFRH320U93::init()?;
    let mut buffer = Buffer::new();
    buffer.write(0x02);
    buffer.write(0x81);

    let speed_byte:u8;

    match speed {
        Speed::S9600 => speed_byte = 0x00,
        Speed::S19200 => speed_byte = 0x01,
        Speed::S38400 => speed_byte = 0x02,
        Speed::S57600 => speed_byte = 0x03,
        Speed::S115200 => speed_byte = 0x04,
    }

    buffer.write(speed_byte);

    device.set_report(buffer.get())?;

    Ok(())
}