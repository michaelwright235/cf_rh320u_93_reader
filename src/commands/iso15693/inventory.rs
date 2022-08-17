use crate::*;

pub fn iso15693_inventory() -> Result<Vec<[u8; 8]>, ReaderError> {
    let device = CFRH320U93::init()?;
    let mut buffer = Buffer::new();
    buffer.write(0x04);
    buffer.write(0x10);
    buffer.write(0x06);
    buffer.write(0x00);
    buffer.write(0x00);

    device.set_report(buffer.get())?;

    let result = device.get_report()?;
    let found = result[11]; // 0x00 - card is present, 0x01 - it's not
    if found == 0x01 {
        return Err(StatusCode::from(result[12]).into())
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
