pub struct Buffer {
    buf: Vec<u8>
}

impl Buffer {
    pub fn new() -> Self {
        Self {buf: Vec::from([0x01,0x00,0x00,0x00,0x00,0x00,0x08,0x00,0xaa,0x00])}
    }
    pub fn write(&mut self, byte: u8) {
        self.buf.push(byte);
    }
    pub fn get(&mut self) -> &Vec<u8> {
        let mut finalize_byte = self.buf[10];
        for i in 11..(self.buf.len()) {
            finalize_byte = finalize_byte ^ self.buf[i];
        }
        self.write(finalize_byte);
        self.write(0xbb);

        for _ in self.buf.len()..256 {
            self.buf.push(0x00);
        }

        &self.buf
    }
}

impl From<[u8; 256]> for Buffer {
    fn from(a: [u8; 256]) -> Self {
        Self {buf: a.to_vec()}
    }
}
