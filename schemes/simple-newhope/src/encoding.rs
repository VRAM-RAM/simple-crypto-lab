use crate::{NewHope, bitwriting::BitWriter};

fn log2_ceil(q: u64) -> u32 {
    if q <= 1 {
        0
    } else {
        64 - (q - 1).leading_zeros()
    }
}

impl NewHope {
    pub fn encode_a(&self, seed: &[u8; 32], b: &Vec<u16>) -> Vec<u8> {
        let mut writer = BitWriter::new(1792 + 32);

        for &byte in seed {
            writer.write_bits(byte as u16, 8);
        }

        for &coeff in b {
            writer.write_bits(coeff, log2_ceil(self.params.q) as usize);
        }

        writer.finish()
    }

    pub fn decode_a(&self, msg: &[u8]) -> ([u8; 32], Vec<u16>) {
        let mut seed = [0u8; 32];

        let mut bit_index = 0;

        let mut read_bits = |bits: usize| -> u16 {
            let mut val = 0u16;
            for i in 0..bits {
                let byte = msg[(bit_index + i) / 8];
                let bit = (byte >> ((bit_index + i) % 8)) & 1;
                val |= (bit as u16) << i;
            }
            bit_index += bits;
            val
        };

        for i in 0..32 {
            seed[i] = read_bits(8) as u8;
        }

        let mut b = Vec::with_capacity(1024);
        for _ in 0..1024 {
            b.push(read_bits(log2_ceil(self.params.q) as usize));
        }

        (seed, b)
    }


    pub fn encode_b(&self, u: &Vec<u16>, r: &Vec<u8>) -> Vec<u8> {
        let mut writer = BitWriter::new(2048);

        for &coeff in u {
            writer.write_bits(coeff, log2_ceil(self.params.q) as usize);
        }

        for &bit in r {
            writer.write_bit(bit);
        }

        writer.finish()
    }

    pub fn decode_b(&self, msg: &[u8]) -> (Vec<u16>, Vec<u8>) {
        let mut bit_index = 0;

        let mut read_bits = |bits: usize| -> u16 {
            let mut val = 0u16;
            for i in 0..bits {
                let byte = msg[(bit_index + i) / 8];
                let bit = (byte >> ((bit_index + i) % 8)) & 1;
                val |= (bit as u16) << i;
            }
            bit_index += bits;
            val
        };

        let mut u = Vec::with_capacity(1024);
        for _ in 0..1024 {
            u.push(read_bits(log2_ceil(self.params.q) as usize));
        }

        let mut r = Vec::with_capacity(1024);
        for _ in 0..1024 {
            r.push(read_bits(1) as u8);
        }

        (u, r)
    }
}