//! Deduplication and content fingerprint helpers.

use blake2::{Blake2b512, Digest};

/// Create a deterministic content fingerprint using a 64-bit simhash.
pub fn content_fingerprint(content: &str) -> String {
    Simhash::new(content).to_hex()
}

/// Basic Charikar-style simhash implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Simhash {
    hash: u64,
}

impl Simhash {
    /// Build a simhash from input text.
    pub fn new(input: &str) -> Self {
        let mut vector = [0i32; 64];
        for token in sample_tokens(input) {
            let token_hash = hash64(&token);
            for (idx, slot) in vector.iter_mut().enumerate() {
                if token_hash & (1u64 << idx) == 0 {
                    *slot -= 1;
                } else {
                    *slot += 1;
                }
            }
        }
        let hash = vector.iter().enumerate().fold(0u64, |acc, (idx, weight)| {
            if *weight >= 0 {
                acc | (1u64 << idx)
            } else {
                acc
            }
        });
        Self { hash }
    }

    /// Build from an existing hash.
    pub fn from_u64(hash: u64) -> Self {
        Self { hash }
    }

    /// Return the raw 64-bit hash.
    pub fn as_u64(self) -> u64 {
        self.hash
    }

    /// Return lowercase hexadecimal representation.
    pub fn to_hex(self) -> String {
        format!("{:016x}", self.hash)
    }

    /// Hamming distance to another simhash.
    pub fn hamming_distance(self, other: Self) -> u32 {
        (self.hash ^ other.hash).count_ones()
    }

    /// Similarity score in range `[0.0, 1.0]`.
    pub fn similarity(self, other: Self) -> f64 {
        f64::from(64 - self.hamming_distance(other)) / 64.0
    }
}

fn sample_tokens(input: &str) -> Vec<String> {
    let tokens: Vec<String> = input
        .split_whitespace()
        .map(|token| {
            token
                .trim_matches(|c: char| c.is_ascii_punctuation())
                .to_lowercase()
        })
        .filter(|token| token.chars().any(char::is_alphanumeric) && token.len() > 2)
        .take(128)
        .collect();

    if tokens.is_empty() {
        input
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>()
            .chars()
            .collect::<Vec<_>>()
            .chunks(4)
            .take(128)
            .map(|chunk| chunk.iter().collect())
            .collect()
    } else {
        tokens
    }
}

fn hash64(input: &str) -> u64 {
    let mut hasher = Blake2b512::new();
    hasher.update(input.as_bytes());
    let digest = hasher.finalize();
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&digest[..8]);
    u64::from_be_bytes(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fingerprint_is_stable() {
        assert_eq!(
            content_fingerprint("hello world"),
            content_fingerprint("hello world")
        );
    }

    #[test]
    fn similar_text_has_high_similarity() {
        let a = Simhash::new("The quick brown fox jumps over the lazy dog");
        let b = Simhash::new("The quick brown fox jumps over a lazy dog");
        assert!(a.similarity(b) > 0.7);
    }
}
