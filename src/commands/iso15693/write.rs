use crate::*;

impl CFRH320U93 {
    /// Writes `data` to a card starting from `skip` byte.
    ///
    /// Only `AccessFlag::WithoutUID` is implemented for now.
    ///
    /// **Warning:** if there's more than one card available,
    /// it's unknown to which one the data will be written to.
    pub fn iso15693_write(
        &self,
        flag: AccessFlag,
        skip: u8,
        data: &[u8],
    ) -> Result<(), ReaderError> {
        let mut checked_data = data.to_vec();

        if flag != AccessFlag::WithoutUID {
            return Err(ReaderError::NotImplemented);
        }

        // The data is written in blocks of 4 bytes.
        // We need to be sure that the given amount of bytes
        // is divisible by 4. Or else we add a couple of 0x00 bytes at the end
        let mut number_of_blocks = (data.len() / 4) as u8;
        let to_add = data.len() % 4;
        if to_add != 0 {
            number_of_blocks += 1;
            for _ in 0..to_add {
                checked_data.push(0x00);
            }
        }

        let mut buffer = Buffer::new();
        buffer.write(0x10);
        buffer.write(0x12);
        buffer.write(flag as u8);
        buffer.write(skip);
        buffer.write(number_of_blocks);

        for b in checked_data {
            buffer.write(b)
        }

        self.set_report(buffer.get())?;

        let result = self.get_report()?;
        let found = result[11]; // 0x00 - if card is present, 0x01 - if it's not
        if found == 0x01 {
            return Err(StatusCode::from(result[12]).into());
        }

        Ok(())
    }
}
