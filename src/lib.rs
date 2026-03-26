use bitvec::prelude::*;
use xxhash_rust::xxh3::xxh3_128;

const K: usize = 10;

pub struct Filter {
    mask: u64,
    array: BitVec<usize, Lsb0>,
}

impl Filter {
    /// Creates a new filter with the given 2^n size.
    pub fn new(size: usize) -> Self {
        Self {
            mask: (size - 1) as u64,
            array: bitvec![0; 1 << size],
        }
    }

    pub fn index(&mut self, s: &str) {
        let (h1, h2) = get_hash_pair(s);

        for i in 0..K as u64 {
            let idx = h1.wrapping_add(i.wrapping_mul(h2));
            self.array.set((idx & self.mask) as usize, true);
        }
    }

    pub fn contains(&self, s: &str) -> bool {
        let (h1, h2) = get_hash_pair(s);

        for i in 0..K as u64 {
            let idx = h1.wrapping_add(i.wrapping_mul(h2));
            if self.array.get((idx & self.mask) as usize).as_deref() == Some(&false) {
                return false;
            }
        }

        true
    }
}

fn get_hash_pair(s: &str) -> (u64, u64) {
    let h128 = xxh3_128(s.as_bytes()).to_le_bytes();
    let h1 = u64::from_le_bytes(h128[0..8].try_into().unwrap());
    let mut h2 = u64::from_le_bytes(h128[8..16].try_into().unwrap());

    h2 |= 1;

    (h1, h2)
}
