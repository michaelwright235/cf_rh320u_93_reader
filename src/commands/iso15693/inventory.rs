use crate::*;

impl CFRH320U93 {
    /// This function requests available cards' ids and
    /// returns a vector of them.
    ///
    /// If something went wrong, `ReaderError` enum is returned.
    /// If there's no cards nearby `ReaderError(StatusCode::NoCard)` is returned.
    pub fn iso15693_inventory(&self) -> Result<Vec<[u8; 8]>, ReaderError> {
        let mut buffer = Buffer::new();
        buffer.write(0x04);
        buffer.write(0x10);
        buffer.write(0x06);
        buffer.write(0x00);
        buffer.write(0x00);

        self.set_report(buffer.get())?;

        let result = self.get_report()?;
        let found = result[11]; // 0x00 - card is present, 0x01 - it's not
        if found == 0x01 {
            return Err(StatusCode::from(result[12]).into());
        }

        let number_of_cards = result[12];

        let mut cards: Vec<[u8; 8]> = vec![];

        for i in 0..number_of_cards {
            let from = 15 + 10 * i;
            let to = from + 8;
            let mut card: [u8; 8] = [0; 8];

            let mut k = 0;
            for j in from..to {
                card[k] = result[j as usize];
                k += 1;
            }
            cards.push(card);
        }

        Ok(cards)
    }
}
