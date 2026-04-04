use crate::error::Error;
use crate::hash::hash;
use bitvec::prelude::*;

pub struct Filter {
    k: u64,
    mask: u64,
    array: BitVec<usize, Lsb0>,
}

impl Filter {
    /// Creates a new filter with 2^`size` bits, optimized for `n` expected items.
    pub fn new(size: usize, n: usize) -> Result<Self, Error> {
        if size > 63 {
            return Err(Error::InvalidArgument("max size is 63".to_string()));
        }

        if n == 0 {
            return Err(Error::InvalidArgument(
                "n must be greater than 0".to_string(),
            ));
        }

        let m = 1u64 << size;
        let k = ((m as f64 / n as f64) * std::f64::consts::LN_2).round() as u64;
        let k = k.max(1);

        Ok(Self {
            k,
            mask: m - 1,
            array: bitvec![0; m as usize],
        })
    }

    pub fn insert(&mut self, value: impl AsRef<[u8]>) {
        let (h1, h2) = hash(value);

        for i in 0..self.k {
            let idx = h1.wrapping_add(i.wrapping_mul(h2));
            let idx = (idx & self.mask) as usize;
            unsafe {
                self.array.set_unchecked(idx, true);
            }
        }
    }

    pub fn contains(&self, value: impl AsRef<[u8]>) -> bool {
        let (h1, h2) = hash(value);

        for i in 0..self.k {
            let idx = h1.wrapping_add(i.wrapping_mul(h2));
            if !self.array[(idx & self.mask) as usize] {
                return false;
            }
        }

        true
    }
}
