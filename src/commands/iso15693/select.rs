use crate::*;

pub fn iso15693_select(flag: AccessFlag, uid: &[u8; 8]) -> Result<(), ReaderError> {
    let device = CFRH320U93::init()?;
    let mut buffer = Buffer::new();
    buffer.write(0x0a);
    buffer.write(0x15);
    buffer.write(flag as u8);

    for u in uid {buffer.write(*u);}

    device.set_report(buffer.get())?;

    let result = device.get_report()?;

    let found = result[11]; // 0x00 - card is present, 0x01 - it's not
    if found == 0x01 {
        return Err(StatusCode::from(result[12]).into())
    }
    if StatusCode::from(result[12]) != StatusCode::Ok {
        return Err(StatusCode::from(result[12]).into());
    }

    Ok(())
}
