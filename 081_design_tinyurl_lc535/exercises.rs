//! Exercises for Design TinyURL (LeetCode 535)
//!
//! # Topics Covered
//! - Base-62 encoding
//! - URL shortening
//! - Hash map for URL storage
//! - O(1) encode/decode operations
//!
//! # Difficulty: Medium

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

/// Codec for URL shortening using base-62 encoding
#[derive(Debug)]
pub struct Codec {
    // Using counter-based approach
    counter: u64,
    short_to_long: HashMap<String, String>,
    long_to_short: HashMap<String, String>,
}

impl Codec {
    pub fn new() -> Self {
        Codec {
            counter: 0,
            short_to_long: HashMap::new(),
            long_to_short: HashMap::new(),
        }
    }

    /// Encodes a URL to a short URL
    pub fn encode(&mut self, long_url: &str) -> String {
        // If already encoded, return existing short URL
        if let Some(short) = self.long_to_short.get(long_url) {
            return short.clone();
        }

        // Generate new short URL
        let id = self.counter;
        self.counter += 1;

        let short_url = id_to_short_url(id);
        let full_short_url = format!("http://tinyurl.com/{}", short_url);

        // Store mappings
        self.short_to_long.insert(short_url.clone(), long_url.to_string());
        self.long_to_short.insert(long_url.to_string(), short_url.clone());

        full_short_url
    }

    /// Decodes a short URL to the original long URL
    pub fn decode(&self, short_url: &str) -> Option<String> {
        // Extract the key from "http://tinyurl.com/{key}"
        let parts: Vec<&str> = short_url.split('/').collect();
        if let Some(key) = parts.last() {
            return self.short_to_long.get(&key.to_string()).cloned();
        }
        None
    }

    /// Returns the number of stored URLs
    pub fn size(&self) -> usize {
        self.short_to_long.len()
    }
}

impl Default for Codec {
    fn default() -> Self {
        Self::new()
    }
}

/// Converts a u64 ID to a base-62 short URL key
fn id_to_short_url(id: u64) -> String {
    const ALPHABET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

    if id == 0 {
        return String::from_utf8(vec![ALPHABET[0]]).unwrap();
    }

    let mut result = Vec::new();
    let mut n = id;

    while n > 0 {
        result.push(ALPHABET[(n % 62) as usize] as char);
        n /= 62;
    }

    result.reverse();
    result.into_iter().collect()
}

/// Converts a base-62 string back to a u64 ID
fn short_url_to_id(short: &str) -> Option<u64> {
    const ALPHABET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

    let mut result: u64 = 0;

    for c in short.chars() {
        let idx = ALPHABET.iter().position(|&x| x as char == c)? as u64;
        result = result * 62 + idx;
    }

    Some(result)
}

/// Alternative codec using hash-based approach
#[derive(Debug)]
pub struct HashCodec {
    short_to_long: HashMap<String, String>,
}

impl HashCodec {
    pub fn new() -> Self {
        HashCodec {
            short_to_long: HashMap::new(),
        }
    }

    /// Encodes a URL using MD5-like hashing
    pub fn encode(&mut self, long_url: &str) -> String {
        let hash = hash_url(long_url);

        // Handle potential collisions by appending if needed
        let mut short = hash;
        let mut counter = 0;
        let original = short.clone();

        while self.short_to_long.contains_key(&short) {
            if self.short_to_long.get(&short) == Some(&long_url.to_string()) {
                // Already have this exact mapping
                break;
            }
            // Collision - try alternative
            short = format!("{}{}", original, counter);
            counter += 1;
        }

        self.short_to_long.insert(short.clone(), long_url.to_string());
        format!("http://tinyurl.com/{}", short)
    }

    /// Decodes a short URL
    pub fn decode(&self, short_url: &str) -> Option<String> {
        let parts: Vec<&str> = short_url.split('/').collect();
        if let Some(key) = parts.last() {
            return self.short_to_long.get(&key.to_string()).cloned();
        }
        None
    }
}

impl Default for HashCodec {
    fn default() -> Self {
        Self::new()
    }
}

/// Hashes a URL to a 6-character base-62 string
fn hash_url(url: &str) -> String {
    let mut hasher = DefaultHasher::new();
    url.hash(&mut hasher);
    let hash = hasher.finish();

    const ALPHABET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

    let mut result = Vec::new();
    let mut h = hash;

    // Generate 6 characters
    for _ in 0..6 {
        result.push(ALPHABET[(h % 62) as usize] as char);
        h /= 62;
    }

    result.reverse();
    result.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_encode_decode() {
        let mut codec = Codec::new();
        let long_url = "https://leetcode.com/problems/design-tinyurl";
        let short_url = codec.encode(long_url);
        assert!(short_url.starts_with("http://tinyurl.com/"));
        assert_eq!(codec.decode(&short_url), Some(long_url.to_string()));
    }

    #[test]
    fn test_encode_decode_multiple() {
        let mut codec = Codec::new();
        let url1 = "https://example.com/path1";
        let url2 = "https://example.com/path2";
        let url3 = "https://example.com/path3";

        let short1 = codec.encode(url1);
        let short2 = codec.encode(url2);
        let short3 = codec.encode(url3);

        assert_ne!(short1, short2);
        assert_ne!(short2, short3);
        assert_ne!(short1, short3);

        assert_eq!(codec.decode(&short1), Some(url1.to_string()));
        assert_eq!(codec.decode(&short2), Some(url2.to_string()));
        assert_eq!(codec.decode(&short3), Some(url3.to_string()));
    }

    #[test]
    fn test_duplicate_url_same_short() {
        let mut codec = Codec::new();
        let long_url = "https://example.com";

        let short1 = codec.encode(long_url);
        let short2 = codec.encode(long_url);

        assert_eq!(short1, short2);
        assert_eq!(codec.size(), 1);
    }

    #[test]
    fn test_different_urls_different_short() {
        let mut codec = Codec::new();
        let url1 = "https://example.com/aaa";
        let url2 = "https://example.com/bbb";

        let short1 = codec.encode(url1);
        let short2 = codec.encode(url2);

        assert_ne!(short1, short2);
    }

    #[test]
    fn test_decode_nonexistent() {
        let codec = Codec::new();
        let result = codec.decode("http://tinyurl.com/nonexistent");
        assert_eq!(result, None);
    }

    #[test]
    fn test_id_to_short_url_conversion() {
        assert_eq!(id_to_short_url(0), "0");
        assert_eq!(id_to_short_url(10), "A");
        assert_eq!(id_to_short_url(62), "10");
        assert_eq!(id_to_short_url(123), "1Z");
        assert_eq!(id_to_short_url(1000), "g8");
    }

    #[test]
    fn test_short_url_to_id_conversion() {
        assert_eq!(short_url_to_id("0"), Some(0));
        assert_eq!(short_url_to_id("A"), Some(10));
        assert_eq!(short_url_to_id("10"), Some(62));
        assert_eq!(short_url_to_id("1Z"), Some(123));
        assert_eq!(short_url_to_id("g8"), Some(1000));
    }

    #[test]
    fn test_round_trip_id_conversion() {
        for id in [0u64, 1, 10, 62, 100, 123, 1000, 9999, 100000, u64::MAX] {
            let short = id_to_short_url(id);
            let decoded = short_url_to_id(&short);
            assert_eq!(decoded, Some(id), "Failed for id {}", id);
        }
    }

    #[test]
    fn test_hash_url_stability() {
        let url = "https://example.com";
        let hash1 = hash_url(url);
        let hash2 = hash_url(url);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_hash_url_different_for_different_urls() {
        let hash1 = hash_url("https://example.com/aaa");
        let hash2 = hash_url("https://example.com/bbb");
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_hash_codec_basic() {
        let mut codec = HashCodec::new();
        let long_url = "https://leetcode.com/problems/design-tinyurl";
        let short_url = codec.encode(long_url);
        assert!(short_url.starts_with("http://tinyurl.com/"));
        assert_eq!(codec.decode(&short_url), Some(long_url.to_string()));
    }

    #[test]
    fn test_hash_codec_decode() {
        let mut codec = HashCodec::new();
        let url1 = "https://example.com/path1";
        let url2 = "https://example.com/path2";

        let short1 = codec.encode(url1);
        let short2 = codec.encode(url2);

        assert_eq!(codec.decode(&short1), Some(url1.to_string()));
        assert_eq!(codec.decode(&short2), Some(url2.to_string()));
    }

    #[test]
    fn test_long_url() {
        let mut codec = Codec::new();
        let long_url = "https://example.com/".repeat(100);
        let short_url = codec.encode(&long_url);
        assert_eq!(codec.decode(&short_url), Some(long_url));
    }

    #[test]
    fn test_url_with_special_chars() {
        let mut codec = Codec::new();
        let url = "https://example.com/path?param1=value1&param2=value2#anchor";
        let short = codec.encode(url);
        assert_eq!(codec.decode(&short), Some(url.to_string()));
    }

    #[test]
    fn test_url_with_unicode() {
        let mut codec = Codec::new();
        let url = "https://example.com/你好";
        let short = codec.encode(url);
        assert_eq!(codec.decode(&short), Some(url.to_string()));
    }

    #[test]
    fn test_empty_path() {
        let mut codec = Codec::new();
        let url = "https://example.com";
        let short = codec.encode(url);
        assert_eq!(codec.decode(&short), Some(url.to_string()));
    }

    #[test]
    fn test_sequential_ids_different() {
        let mut codec = Codec::new();
        let url1 = "https://a.com/1";
        let url2 = "https://a.com/2";
        let url3 = "https://a.com/3";

        let short1 = codec.encode(url1);
        let short2 = codec.encode(url2);
        let short3 = codec.encode(url3);

        // Each should be unique
        assert_ne!(short1, short2);
        assert_ne!(short2, short3);
        assert_ne!(short1, short3);

        // All should decode correctly
        assert_eq!(codec.decode(&short1), Some(url1.to_string()));
        assert_eq!(codec.decode(&short2), Some(url2.to_string()));
        assert_eq!(codec.decode(&short3), Some(url3.to_string()));
    }

    #[test]
    fn test_size_after_operations() {
        let mut codec = Codec::new();
        assert_eq!(codec.size(), 0);

        codec.encode("https://example.com/1");
        assert_eq!(codec.size(), 1);

        codec.encode("https://example.com/2");
        assert_eq!(codec.size(), 2);

        codec.encode("https://example.com/1"); // duplicate
        assert_eq!(codec.size(), 2); // should not increase
    }

    #[test]
    fn test_many_encodes() {
        let mut codec = Codec::new();
        // Encode 1000 URLs
        for i in 0..1000 {
            let url = format!("https://example.com/{}", i);
            let short = codec.encode(&url);
            assert!(short.starts_with("http://tinyurl.com/"));
        }
        assert_eq!(codec.size(), 1000);
    }

    #[test]
    fn test_short_url_format() {
        let mut codec = Codec::new();
        let short = codec.encode("https://example.com");
        let parts: Vec<&str> = short.split('/').collect();
        let key = parts.last().unwrap();
        // Key should be base-62 characters
        assert!(key.chars().all(|c| c.is_ascii_alphanumeric()));
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("Design TinyURL exercises - run tests with cargo test");
}