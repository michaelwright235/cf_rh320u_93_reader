use crate::*;

impl CFRH320U93 {
    /// Turns the reader's buzzer on `freq` times for `duration`.
    pub fn control_buzzer(&self, freq: u8, duration: u8) -> Result<(), ReaderError> {
        let mut buffer = Buffer::new();
        buffer.write(0x03);
        buffer.write(0x89);
        buffer.write(duration);
        buffer.write(freq);

        self.set_report(buffer.get())?; 

        let read_buf = self.get_report()?;

        if StatusCode::from(read_buf[12]) != StatusCode::Ok {
            return Err(StatusCode::from(read_buf[12]).into());
        }

        Ok(())
    }
}
