use crate::{Polynomial, RingParams, bitwriting::BitWriter};

fn log2_ceil(q: u64) -> u32 {
    if q <= 1 {
        0
    } else {
        64 - (q - 1).leading_zeros()
    }
}

impl RingParams {
    pub fn encode_a(&self, seed: &[u8; 32], b: &Polynomial) -> Vec<u8> {
        let mut writer = BitWriter::new(1792 + 32);

        for &byte in seed {
            writer.write_bits(byte as u64, 8);
        }

        for coeff in b.coeffs.clone() {
            writer.write_bits(coeff, log2_ceil(self.q) as usize);
        }

        writer.finish()
    }

    pub fn decode_a(&self, msg: &[u8]) -> ([u8; 32], Polynomial) {
        let mut seed = [0u8; 32];

        let mut bit_index = 0;

        let mut read_bits = |bits: usize| -> u64 {
            let mut val = 0u64;
            for i in 0..bits {
                let byte = msg[(bit_index + i) / 8];
                let bit = (byte >> ((bit_index + i) % 8)) & 1;
                val |= (bit as u64) << i;
            }
            bit_index += bits;
            val
        };

        for i in 0..32 {
            seed[i] = read_bits(8) as u8;
        }

        let mut b = Vec::with_capacity(self.n);
        for _ in 0..self.n {
            b.push(read_bits(log2_ceil(self.q) as usize));
        }

        (seed, Polynomial::new(b))
    }


    pub fn encode_b(&self, u: &Polynomial, r: &Vec<u8>) -> Vec<u8> {
        let mut writer = BitWriter::new(2048);

        for coeff in u.coeffs.clone() {
            writer.write_bits(coeff, log2_ceil(self.q) as usize);
        }

        for &bit in r {
            writer.write_bit(bit);
        }

        writer.finish()
    }

    pub fn decode_b(&self, msg: &[u8]) -> (Polynomial, Vec<u8>) {
        let mut bit_index = 0;

        let mut read_bits = |bits: usize| -> u64 {
            let mut val = 0u64;
            for i in 0..bits {
                let byte = msg[(bit_index + i) / 8];
                let bit = (byte >> ((bit_index + i) % 8)) & 1;
                val |= (bit as u64) << i;
            }
            bit_index += bits;
            val
        };

        let mut u = Vec::with_capacity(self.n);
        for _ in 0..self.n {
            u.push(read_bits(log2_ceil(self.q) as usize));
        }

        let mut r = Vec::with_capacity(self.n);
        for _ in 0..self.n {
            r.push(read_bits(1) as u8);
        }

        (Polynomial::new(u), r)
    }
}