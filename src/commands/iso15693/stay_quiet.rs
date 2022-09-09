use crate::*;

impl CFRH320U93 {
    pub fn iso15693_stay_quiet(&self, flag: AccessFlag, uid: &[u8; 8]) -> Result<(), ReaderError> {
        let mut buffer = Buffer::new();
        buffer.write(0x10);
        buffer.write(0x14);
        buffer.write(flag as u8);

        for u in uid {buffer.write(*u);}

        self.set_report(buffer.get())?;

        let result = self.get_report()?;

        let found = result[11]; // 0x00 - card is present, 0x01 - it's not
        if found == 0x01 {
            return Err(StatusCode::from(result[12]).into())
        }
        if StatusCode::from(result[12]) != StatusCode::Ok {
            return Err(StatusCode::from(result[12]).into());
        }

        Ok(())
    }
}
