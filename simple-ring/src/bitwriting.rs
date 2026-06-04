pub struct BitWriter {
    buf: Vec<u8>,
    acc: u8,
    filled: u8,
}

impl BitWriter {
    pub fn new(capacity_bytes: usize) -> Self {
        Self {
            buf: Vec::with_capacity(capacity_bytes),
            acc: 0,
            filled: 0,
        }
    }

    pub fn write_bit(&mut self, bit: u8) {
        self.acc |= (bit & 1) << self.filled;
        self.filled += 1;

        if self.filled == 8 {
            self.buf.push(self.acc);
            self.acc = 0;
            self.filled = 0;
        }
    }

    pub fn write_bits(&mut self, mut value: u64, bits: usize) {
        for _ in 0..bits {
            self.write_bit((value & 1) as u8);
            value >>= 1;
        }
    }

    pub fn finish(mut self) -> Vec<u8> {
        if self.filled > 0 {
            self.buf.push(self.acc);
        }
        self.buf
    }
}