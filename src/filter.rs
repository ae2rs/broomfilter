use crate::error::Error;
use crate::hash::hash;

#[derive(Clone, Debug)]
pub struct Filter {
    k: u64,
    mask: u64,
    array: Box<[u64]>,
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
        let k = k.clamp(1, 30);
        let words = ((m as usize) + 63) >> 6;

        Ok(Self {
            k,
            mask: m - 1,
            array: vec![0u64; words].into_boxed_slice(),
        })
    }

    /// Creates a new filter optimized for `n` expected items with a desired
    /// false positive rate `fpr` (e.g. 0.01 for 1%).
    ///
    /// The filter size is rounded up to the next power of 2, so the actual
    /// false positive rate may be significantly better than requested (at the
    /// cost of extra memory). Use [`from_fpr_exact`](Self::from_fpr_exact)
    /// for a tighter memory fit with non-power-of-2 sizing.
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

        // m = -n * ln(fpr) / (ln(2)^2), rounded up to next power of 2
        let m_raw = -(n as f64) * fpr.ln() / (std::f64::consts::LN_2.powi(2));
        let size = (m_raw.log2().ceil() as usize).max(1);

        Self::new(size, n)
    }

    #[inline]
    pub fn insert(&mut self, value: impl AsRef<[u8]>) {
        let (h1, h2) = hash(value);

        for i in 0..self.k {
            let idx = h1.wrapping_add(i.wrapping_mul(h2)) & self.mask;
            // SAFETY: mask guarantees idx < m, and m <= array.len() * 64.
            unsafe {
                let word = self.array.get_unchecked_mut((idx >> 6) as usize);
                *word |= 1 << (idx & 63);
            }
        }
    }

    #[inline]
    pub fn contains(&self, value: impl AsRef<[u8]>) -> bool {
        let (h1, h2) = hash(value);

        for i in 0..self.k {
            let idx = h1.wrapping_add(i.wrapping_mul(h2)) & self.mask;
            // SAFETY: mask guarantees idx < m, and m <= array.len() * 64.
            unsafe {
                if *self.array.get_unchecked((idx >> 6) as usize) & (1 << (idx & 63)) == 0 {
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
        let m = (self.mask + 1) as f64;
        let bits_set: u64 = self.array.iter().map(|w| w.count_ones() as u64).sum();
        -(m / self.k as f64) * (1.0 - bits_set as f64 / m).ln()
    }

    /// Merges another filter into this one (bitwise OR).
    ///
    /// Both filters must have been created with identical parameters.
    pub fn union(&mut self, other: &Filter) -> Result<(), Error> {
        if self.k != other.k || self.mask != other.mask {
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
        assert_send_sync::<Filter>();
    }
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_false_negatives() {
        let n = 1000;
        let mut filter = Filter::new(14, n).unwrap();

        for i in 0..n {
            filter.insert(i.to_string());
        }

        for i in 0..n {
            assert!(filter.contains(i.to_string()), "false negative for {i}");
        }
    }

    #[test]
    fn fpr_within_bounds() {
        let n = 10_000;
        let mut filter = Filter::new(17, n).unwrap();

        for i in 0..n {
            filter.insert(format!("member-{i}"));
        }

        let test_count = 100_000;
        let false_positives = (0..test_count)
            .filter(|i| filter.contains(format!("absent-{i}")))
            .count();

        let fpr = false_positives as f64 / test_count as f64;
        // Theoretical FPR for m=131072, n=10000, k=9 is ~0.36%.
        // Allow generous headroom.
        assert!(fpr < 0.02, "FPR too high: {fpr:.4}");
    }

    #[test]
    fn single_item() {
        let mut filter = Filter::new(10, 1).unwrap();
        filter.insert("only");
        assert!(filter.contains("only"));
        assert!(!filter.contains("other"));
    }

    #[test]
    fn small_k() {
        // size=1 gives m=2, n=1000 → k=1
        let mut filter = Filter::new(1, 1000).unwrap();
        filter.insert("a");
        assert!(filter.contains("a"));
    }

    #[test]
    fn large_k() {
        // size=20 gives m=1048576, n=1 → very large k (capped by m/n * ln2)
        let mut filter = Filter::new(20, 1).unwrap();
        filter.insert("a");
        assert!(filter.contains("a"));
    }

    #[test]
    fn empty_filter_contains_nothing() {
        let filter = Filter::new(10, 100).unwrap();
        for i in 0..1000 {
            assert!(!filter.contains(i.to_string()));
        }
    }

    #[test]
    fn from_fpr_no_false_negatives() {
        let n = 1000;
        let mut filter = Filter::from_fpr(n, 0.01).unwrap();

        for i in 0..n {
            filter.insert(i.to_string());
        }

        for i in 0..n {
            assert!(filter.contains(i.to_string()), "false negative for {i}");
        }
    }

    #[test]
    fn from_fpr_respects_target_rate() {
        let n = 5000;
        let target_fpr = 0.01;
        let mut filter = Filter::from_fpr(n, target_fpr).unwrap();

        for i in 0..n {
            filter.insert(format!("member-{i}"));
        }

        let test_count = 100_000;
        let false_positives = (0..test_count)
            .filter(|i| filter.contains(format!("absent-{i}")))
            .count();

        let fpr = false_positives as f64 / test_count as f64;
        // Power-of-2 rounding may overshoot slightly; allow 2x the target.
        assert!(
            fpr < target_fpr * 2.0,
            "FPR {fpr:.4} exceeds 2x target {target_fpr}"
        );
    }

    #[test]
    fn new_rejects_invalid_args() {
        assert!(Filter::new(64, 100).is_err());
        assert!(Filter::new(10, 0).is_err());
    }

    #[test]
    fn from_fpr_rejects_invalid_args() {
        assert!(Filter::from_fpr(0, 0.01).is_err());
        assert!(Filter::from_fpr(100, 0.0).is_err());
        assert!(Filter::from_fpr(100, 1.0).is_err());
        assert!(Filter::from_fpr(100, -0.5).is_err());
    }

    #[test]
    fn clear_resets_filter() {
        let mut filter = Filter::new(10, 100).unwrap();
        for i in 0..100 {
            filter.insert(i.to_string());
        }
        filter.clear();
        for i in 0..100 {
            assert!(!filter.contains(i.to_string()));
        }
    }

    #[test]
    fn estimated_count_approximates_insertions() {
        let n = 500;
        let mut filter = Filter::new(14, n).unwrap();
        for i in 0..n {
            filter.insert(i.to_string());
        }
        let estimate = filter.estimated_count();
        let error = (estimate - n as f64).abs() / n as f64;
        assert!(error < 0.1, "estimate {estimate:.0} too far from {n}");
    }

    #[test]
    fn union_merges_filters() {
        let mut a = Filter::new(10, 100).unwrap();
        let mut b = Filter::new(10, 100).unwrap();
        a.insert("hello");
        b.insert("world");
        a.union(&b).unwrap();
        assert!(a.contains("hello"));
        assert!(a.contains("world"));
    }

    #[test]
    fn union_rejects_mismatched_filters() {
        let mut a = Filter::new(10, 100).unwrap();
        let b = Filter::new(12, 100).unwrap();
        assert!(a.union(&b).is_err());
    }
}
