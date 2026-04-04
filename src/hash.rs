use xxhash_rust::xxh3::xxh3_128;

#[inline]
pub fn hash(value: impl AsRef<[u8]>) -> (u64, u64) {
    let h128 = xxh3_128(value.as_ref());
    let h1 = h128 as u64;
    let mut h2 = (h128 >> 64) as u64;
    h2 |= 1;
    (h1, h2)
}
