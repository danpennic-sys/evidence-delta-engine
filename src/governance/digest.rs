//! Content digest for pending CEP binding (no external crypto crate required).

/// Stable hex digest of bytes (FNV-1a 64 + length prefix - integrity, not secrecy).
pub fn content_digest(bytes: &[u8]) -> String {
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;
    let mut hash = FNV_OFFSET;
    for b in bytes {
        hash ^= u64::from(*b);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    // Mix in length to avoid trivial collisions on truncation.
    hash ^= bytes.len() as u64;
    hash = hash.wrapping_mul(FNV_PRIME);
    format!("{hash:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn digest_stable() {
        assert_eq!(content_digest(b"abc"), content_digest(b"abc"));
        assert_ne!(content_digest(b"abc"), content_digest(b"abd"));
    }
}
