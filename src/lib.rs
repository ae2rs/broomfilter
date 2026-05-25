mod blocked;
mod error;
mod filter;
mod hash;

pub use blocked::BlockedFilter;
pub use error::Error;
pub use filter::Filter;

#[cfg(feature = "benchmarks")]
pub fn hash_bytes(value: &[u8]) -> (u64, u64) {
    hash::hash(value)
}
