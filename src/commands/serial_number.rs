use crate::*;

impl CFRH320U93 {
    /// Requests the reader's internal serial number. 
    pub fn internal_serial_number(&self) -> Result<[u8; 8], ReaderError> {
        let mut buffer = Buffer::new();
        buffer.write(0x01);
        buffer.write(0x83);

        self.set_report(buffer.get())?;

        let result = self.get_report()?;
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

    /// Sets the reader's internal serial number. 
    pub fn set_internal_serial_number(&self, serial_number: &[u8; 8]) -> Result<(), ReaderError> {
        let mut buffer = Buffer::new();
        buffer.write(0x09);
        buffer.write(0x82);
        for x in serial_number {
            buffer.write(*x);
        }

        self.set_report(buffer.get())?;

        let result = self.get_report()?;

        if StatusCode::from(result[12]) != StatusCode::Ok {
            return Err(StatusCode::from(result[12]).into());
        }

        Ok(())
    }

    /// Requests the reader's version number. 
    pub fn version_number(&self) -> Result<[u8; 12], ReaderError> {
        let mut buffer = Buffer::new();
        buffer.write(0x01);
        buffer.write(0x86);

        self.set_report(buffer.get())?;

        let result = self.get_report()?;

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
}
