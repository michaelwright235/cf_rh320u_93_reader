const BUFFER_START: [u8; 10] = [0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0xaa, 0x00];

pub(crate) struct Buffer {
    buf: Vec<u8>,
}

impl Buffer {
    pub fn new() -> Self {
        let mut vector = Vec::with_capacity(256);
        vector.append(&mut Vec::from(BUFFER_START));
        Self { buf: vector }
    }
    pub fn write(&mut self, byte: u8) {
        self.buf.push(byte);
    }
    pub fn get(&mut self) -> &Vec<u8> {
        let mut finalize_byte = self.buf[10];
        for i in 11..(self.buf.len()) {
            finalize_byte ^= self.buf[i];
        }
        self.write(finalize_byte);
        self.write(0xbb);

        for _ in self.buf.len()..256 {
            self.buf.push(0x00);
        }

        &self.buf
    }
}
