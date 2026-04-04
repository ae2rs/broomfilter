use crate::error::Error;
use crate::hash::hash;

/// Number of `u64` words per cache-line block (64 bytes = 8 × u64).
const BLOCK_WORDS: usize = 8;
/// Number of bits per block (512).
const BLOCK_BITS: u64 = (BLOCK_WORDS as u64) * 64;

/// A cache-line-blocked bloom filter.
///
/// All `k` bit probes for a given item land in the same 512-bit (64-byte)
/// block, guaranteeing at most one cache miss per lookup. This trades a
/// small amount of FPR accuracy for significantly better cache behavior
/// on large filters.
#[derive(Clone, Debug)]
pub struct BlockedFilter {
    k: u64,
    num_blocks: u64,
    array: Box<[u64]>,
}

impl BlockedFilter {
    /// Creates a new blocked filter with the given number of `bits`,
    /// optimized for `n` expected items.
    ///
    /// `bits` is rounded up to a multiple of 512 (one cache line).
    pub fn new(bits: usize, n: usize) -> Result<Self, Error> {
        if bits == 0 {
            return Err(Error::InvalidArgument(
                "bits must be greater than 0".to_string(),
            ));
        }
        if n == 0 {
            return Err(Error::InvalidArgument(
                "n must be greater than 0".to_string(),
            ));
        }

        let num_blocks = (bits as u64).div_ceil(BLOCK_BITS);
        let num_blocks = num_blocks.max(1);
        let total_words = num_blocks as usize * BLOCK_WORDS;

        // k is computed from the effective total bits.
        let m = num_blocks * BLOCK_BITS;
        let k = ((m as f64 / n as f64) * std::f64::consts::LN_2).round() as u64;
        let k = k.clamp(1, 30);

        Ok(Self {
            k,
            num_blocks,
            array: vec![0u64; total_words].into_boxed_slice(),
        })
    }

    /// Creates a new blocked filter optimized for `n` expected items with
    /// a desired false positive rate `fpr`.
    pub fn from_fpr(n: usize, fpr: f64) -> Result<Self, Error> {
        if n == 0 {
            return Err(Error::InvalidArgument(
                "n must be greater than 0".to_string(),
            ));
        }
        if fpr <= 0.0 || fpr >= 1.0 {
            return Err(Error::InvalidArgument(
                "fpr must be between 0 and 1 exclusive".to_string(),
            ));
        }

        let m = (-(n as f64) * fpr.ln() / (std::f64::consts::LN_2.powi(2))).ceil() as usize;
        Self::new(m, n)
    }

    #[inline]
    fn probe_masks(h1: u64, h2: u64, k: u64) -> [u64; BLOCK_WORDS] {
        // Use h2 as base and an odd step derived from both halves for
        // good intra-block distribution.
        let step = h1.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
        let mut masks = [0u64; BLOCK_WORDS];
        for i in 0..k {
            let bit = h2.wrapping_add(i.wrapping_mul(step)) & (BLOCK_BITS - 1);
            masks[(bit >> 6) as usize] |= 1 << (bit & 63);
        }
        masks
    }

    #[inline]
    pub fn insert(&mut self, value: impl AsRef<[u8]>) {
        let (h1, h2) = hash(value);
        let block_idx = (h1 % self.num_blocks) as usize * BLOCK_WORDS;
        let masks = Self::probe_masks(h1, h2, self.k);

        // SAFETY: block_idx is at most (num_blocks - 1) * BLOCK_WORDS,
        // so block_idx + BLOCK_WORDS <= array.len().
        unsafe {
            for (j, &mask) in masks.iter().enumerate() {
                *self.array.get_unchecked_mut(block_idx + j) |= mask;
            }
        }
    }

    #[inline]
    pub fn contains(&self, value: impl AsRef<[u8]>) -> bool {
        let (h1, h2) = hash(value);
        let block_idx = (h1 % self.num_blocks) as usize * BLOCK_WORDS;
        let masks = Self::probe_masks(h1, h2, self.k);

        // SAFETY: same bound as insert.
        unsafe {
            for (j, &mask) in masks.iter().enumerate() {
                if *self.array.get_unchecked(block_idx + j) & mask != mask {
                    return false;
                }
            }
        }

        true
    }

    /// Resets the filter to empty without reallocating.
    pub fn clear(&mut self) {
        self.array.fill(0);
    }

    /// Estimates the number of distinct items currently in the filter.
    pub fn estimated_count(&self) -> f64 {
        let m = (self.num_blocks * BLOCK_BITS) as f64;
        let bits_set: u64 = self.array.iter().map(|w| w.count_ones() as u64).sum();
        -(m / self.k as f64) * (1.0 - bits_set as f64 / m).ln()
    }

    /// Merges another filter into this one (bitwise OR).
    ///
    /// Both filters must have been created with identical parameters.
    pub fn union(&mut self, other: &BlockedFilter) -> Result<(), Error> {
        if self.k != other.k || self.num_blocks != other.num_blocks {
            return Err(Error::InvalidArgument(
                "filters must have identical parameters".to_string(),
            ));
        }
        for (a, b) in self.array.iter_mut().zip(other.array.iter()) {
            *a |= *b;
        }
        Ok(())
    }
}

const _: () = {
    #[allow(dead_code)]
    fn assert_send_sync<T: Send + Sync>() {}
    #[allow(dead_code)]
    fn check() {
        assert_send_sync::<BlockedFilter>();
    }
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_false_negatives() {
        let n = 1000;
        let mut filter = BlockedFilter::new(16384, n).unwrap();
        for i in 0..n {
            filter.insert(i.to_string());
        }
        for i in 0..n {
            assert!(filter.contains(i.to_string()), "false negative for {i}");
        }
    }

    #[test]
    fn fpr_within_bounds() {
        let n = 1_000;
        // ~20 bits/item gives blocked filters plenty of room.
        let mut filter = BlockedFilter::new(20_000, n).unwrap();
        for i in 0..n {
            filter.insert(format!("member-{i}"));
        }
        let test_count = 100_000;
        let false_positives = (0..test_count)
            .filter(|i| filter.contains(format!("absent-{i}")))
            .count();
        let fpr = false_positives as f64 / test_count as f64;
        assert!(fpr < 0.02, "FPR too high: {fpr:.4}");
    }

    #[test]
    fn from_fpr_no_false_negatives() {
        let n = 1000;
        let mut filter = BlockedFilter::from_fpr(n, 0.01).unwrap();
        for i in 0..n {
            filter.insert(i.to_string());
        }
        for i in 0..n {
            assert!(filter.contains(i.to_string()), "false negative for {i}");
        }
    }

    #[test]
    fn empty_filter_contains_nothing() {
        let filter = BlockedFilter::new(1024, 100).unwrap();
        for i in 0..1000 {
            assert!(!filter.contains(i.to_string()));
        }
    }

    #[test]
    fn clear_resets_filter() {
        let mut filter = BlockedFilter::new(1024, 100).unwrap();
        for i in 0..100 {
            filter.insert(i.to_string());
        }
        filter.clear();
        for i in 0..100 {
            assert!(!filter.contains(i.to_string()));
        }
    }

    #[test]
    fn union_merges_filters() {
        let mut a = BlockedFilter::new(1024, 100).unwrap();
        let mut b = BlockedFilter::new(1024, 100).unwrap();
        a.insert("hello");
        b.insert("world");
        a.union(&b).unwrap();
        assert!(a.contains("hello"));
        assert!(a.contains("world"));
    }

    #[test]
    fn union_rejects_mismatched_filters() {
        let mut a = BlockedFilter::new(1024, 100).unwrap();
        let b = BlockedFilter::new(2048, 100).unwrap();
        assert!(a.union(&b).is_err());
    }

    #[test]
    fn rejects_invalid_args() {
        assert!(BlockedFilter::new(0, 100).is_err());
        assert!(BlockedFilter::new(1024, 0).is_err());
        assert!(BlockedFilter::from_fpr(0, 0.01).is_err());
        assert!(BlockedFilter::from_fpr(100, 0.0).is_err());
        assert!(BlockedFilter::from_fpr(100, 1.0).is_err());
    }
}
