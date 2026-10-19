//! Integer-key hashing for internal chart identities and finite summaries.

use std::hash::Hasher;

#[derive(Default)]
pub(crate) struct ChartHasher(u64);

impl ChartHasher {
    fn mix(&mut self, value: u64) {
        self.0 = (self.0.rotate_left(5) ^ value).wrapping_mul(0x517c_c1b7_2722_0a95);
    }
}

impl Hasher for ChartHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        let (chunks, remainder) = bytes.as_chunks::<8>();
        for &chunk in chunks {
            self.mix(u64::from_ne_bytes(chunk));
        }
        for &byte in remainder {
            self.mix(u64::from(byte));
        }
    }

    fn write_u8(&mut self, value: u8) {
        self.mix(u64::from(value));
    }
    fn write_u16(&mut self, value: u16) {
        self.mix(u64::from(value));
    }
    fn write_u32(&mut self, value: u32) {
        self.mix(u64::from(value));
    }
    fn write_u64(&mut self, value: u64) {
        self.mix(value);
    }
    fn write_usize(&mut self, value: usize) {
        self.mix(value as u64);
    }
}
