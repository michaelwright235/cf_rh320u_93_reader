use crate::*;

pub fn iso15693_read(flag: AccessFlag, skip: u8, number_of_blocks: u8) -> Result<Vec<u8>, ReaderError> {
    let device = CFRH320U93::init()?;
    let mut buffer = Buffer::new();
    buffer.write(0x04);
    buffer.write(0x11);
    buffer.write(flag as u8);
    buffer.write(skip);
    buffer.write(number_of_blocks);

    // reader accepts uid but doesn't care about it. so it returns the data of whatever card is present
    //let uid = [0x00, 0x00, 0x68, 0x0E, 0x4E, 0x38, 0x08, 0x01, 0x04, 0xE0];
    //for u in uid {buffer.write(u);}

    device.set_report(buffer.get())?;

    let result = device.get_report()?;
    let found = result[11]; // 0x00 - card is present, 0x01 - it's not
    if found == 0x01 {
        return Err(StatusCode::from(result[12]).into())
    }
    let data = result[13..result.len()].to_vec();

    Ok(data)
}
