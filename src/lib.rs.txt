use bytebuffer::ByteBuffer;

pub struct ByteBufferAdv(ByteBuffer);

impl ByteBufferAdv {
    pub fn new() -> Self {
        ByteBufferAdv(ByteBuffer::new())
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub fn write_bytes(&mut self, bytes:&[u8] ) {
        self.0.write_bytes(bytes);
    }
}

impl ByteBufferAdv {
    pub fn write_pos(&mut self, data: &[u8], wpos: usize) {
        if wpos > self.0.len() {
            panic!("Write pos in buffer exceeds buffer length");
        }
        let next_write_pos = if (wpos + data.len() > self.0.get_wpos()) {
            wpos + data.len()
        } else {
            self.0.get_wpos()
        };
        self.0.set_wpos(wpos);
        self.0.write_bytes(data);
        self.0.set_wpos(next_write_pos);
    }

    pub fn write_u8_pos(&mut self, data: u8, wpos: usize) {
        if wpos >= self.0.len() {
            panic!("Write pos in buffer exceeds buffer length");
        }
        let next_write_pos = self.0.get_wpos();
        self.0.set_wpos(wpos);
        self.0.write_u8(data);
        self.0.set_wpos(next_write_pos);
    }

    pub fn write_u16_pos(&mut self, data: u16, wpos: usize) {
        if wpos + 1 >= self.0.len() {
            panic!("Write pos in buffer exceeds buffer length");
        }
        let next_write_pos = self.0.get_wpos();
        self.0.set_wpos(wpos);
        self.0.write_u16(data);
        self.0.set_wpos(next_write_pos);
    }

    pub fn write_u32_pos(&mut self, data: u32, wpos: usize) {
        if wpos + 3 >= self.0.len() {
            panic!("Write pos in buffer exceeds buffer length");
        }
        let next_write_pos = self.0.get_wpos();
        self.0.set_wpos(wpos);
        self.0.write_u32(data);
        self.0.set_wpos(next_write_pos);
    }

    pub fn write_u64_pos(&mut self, data: u64, wpos: usize) {
        if wpos + 7 >= self.0.len() {
            panic!("Write pos in buffer exceeds buffer length");
        }
        let next_write_pos = self.0.get_wpos();
        self.0.set_wpos(wpos);
        self.0.write_u64(data);
        self.0.set_wpos(next_write_pos);
    }

    pub fn write_u128_pos(&mut self, data: u128, wpos: usize) {
        if wpos + 15 >= self.0.len() {
            panic!("Write pos in buffer exceeds buffer length");
        }
        let next_write_pos = self.0.get_wpos();
        self.0.set_wpos(wpos);
        self.0.write_u128(data);
        self.0.set_wpos(next_write_pos);
    }

    pub fn write_i8_pos(&mut self, data: i8, wpos: usize) {
        self.write_u8_pos(data as u8, wpos);
    }

    pub fn write_i16_pos(&mut self, data: i16, wpos: usize) {
        self.write_u16_pos(data as u16, wpos);
    }

    pub fn write_i32_pos(&mut self, data: i32, wpos: usize) {
        self.write_u32_pos(data as u32, wpos);
    }

    pub fn write_i64_pos(&mut self, data: i64, wpos: usize) {
        self.write_u64_pos(data as u64, wpos);
    }

    pub fn write_i128_pos(&mut self, data: i128, wpos: usize) {
        self.write_u128_pos(data as u128, wpos);
    }
}
