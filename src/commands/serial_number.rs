use crate::*;

pub fn get_internal_serial_number() -> Result<[u8; 8], ReaderError> {
    let device = CFRH320U93::init()?;
    let mut buffer = Buffer::new();
    buffer.write(0x01);
    buffer.write(0x83);

    device.set_report(buffer.get())?;

    let result = device.get_report()?;
    let mut num = [0; 8];

    if result.len() < 21 {
        return Err(StatusCode::InvalidData.into()); // todo: return actual error
    }
    let mut i = 0;
    for x in &result[13..21] {
        num[i] = *x;
        i+=1;
    }
    Ok(num)
}

pub fn set_internal_serial_number(serial_number: &[u8; 8]) -> Result<(), ReaderError> {
    let device = CFRH320U93::init()?;
    let mut buffer = Buffer::new();
    buffer.write(0x09);
    buffer.write(0x82);
    for x in serial_number {
        buffer.write(*x);
    }

    device.set_report(buffer.get())?;

    let result = device.get_report()?;

    if StatusCode::from(result[12]) != StatusCode::Ok {
        return Err(StatusCode::from(result[12]).into());
    }

    Ok(())
}

pub fn get_version_number() -> Result<[u8; 12], ReaderError> {
    let device = CFRH320U93::init()?;
    let mut buffer = Buffer::new();
    buffer.write(0x01);
    buffer.write(0x86);

    device.set_report(buffer.get())?;

    let result = device.get_report()?;

    if result.len() < 24 {
        return Err(StatusCode::InvalidData.into()); // todo: return actual error
    }

    let mut num = [0; 12];
    let mut i = 0;
    for x in &result[12..24] { // todo: determine how the end is defined
        num[i] = *x;
        i+=1;
    }

    Ok(num)
}
