use xxhash_rust::xxh3::xxh3_128;

pub fn hash(value: impl AsRef<[u8]>) -> (u64, u64) {
    let h128 = xxh3_128(value.as_ref()).to_le_bytes();
    let h1 = u64::from_le_bytes(h128[0..8].try_into().unwrap());
    let mut h2 = u64::from_le_bytes(h128[8..16].try_into().unwrap());

    h2 |= 1;

    (h1, h2)
}
