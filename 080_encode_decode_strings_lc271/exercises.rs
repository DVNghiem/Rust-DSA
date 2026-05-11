//! Exercises for Encode and Decode Strings (LeetCode 271)
//!
//! # Topics Covered
//! - String encoding/decoding
//! - Length prefix encoding
//! - Variable-length data handling
//! - UTF-8 byte handling
//!
//! # Difficulty: Medium

/// Encodes a list of strings into a single string
/// Format: [length]:[string] for each string
pub fn encode(strs: Vec<String>) -> String {
    let mut encoded = String::new();
    for s in &strs {
        encoded.push_str(&s.len().to_string());
        encoded.push(':');
        encoded.push_str(s);
    }
    encoded
}

/// Decodes a single string back to list of strings
pub fn decode(encoded: &str) -> Vec<String> {
    let mut result = Vec::new();
    let bytes = encoded.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        // Find colon position
        let mut colon_pos = i;
        while colon_pos < bytes.len() && bytes[colon_pos] != b':' {
            colon_pos += 1;
        }

        if colon_pos >= bytes.len() {
            break; // Invalid encoding
        }

        // Parse length
        let len_str = std::str::from_utf8(&bytes[i..colon_pos]).unwrap_or("");
        let len: usize = len_str.parse().unwrap_or(0);

        // Extract the string
        let start = colon_pos + 1;
        if start + len > bytes.len() {
            break; // Invalid encoding
        }

        let s = std::str::from_utf8(&bytes[start..start + len]).unwrap_or("");
        result.push(s.to_string());

        i = start + len;
    }
    result
}

/// Alternative encode using format!
pub fn encode_v2(strs: Vec<String>) -> String {
    strs.into_iter()
        .map(|s| format!("{}:{}", s.len(), s))
        .collect::<Vec<_>>()
        .join("")
}

/// Alternative decode with find() and parse()
pub fn decode_v2(encoded: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < encoded.len() {
        if let Some(colon_pos) = encoded[i..].find(':') {
            let actual_colon = i + colon_pos;
            if let Ok(len) = encoded[i..actual_colon].parse::<usize>() {
                let start = actual_colon + 1;
                let end = start + len;
                if end <= encoded.len() {
                    result.push(encoded[start..end].to_string());
                    i = end;
                    continue;
                }
            }
        }
        break; // Invalid
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode_basic() {
        let strs = vec!["hello".to_string(), "world".to_string()];
        let encoded = encode(strs.clone());
        let decoded = decode(&encoded);
        assert_eq!(decoded, strs);
    }

    #[test]
    fn test_encode_decode_multiple() {
        let strs = vec![
            "lint".to_string(),
            "code".to_string(),
            "love".to_string(),
            "you".to_string(),
        ];
        let encoded = encode(strs.clone());
        let decoded = decode(&encoded);
        assert_eq!(decoded, strs);
    }

    #[test]
    fn test_empty_list() {
        let strs: Vec<String> = vec![];
        let encoded = encode(strs);
        let decoded = decode(&encoded);
        assert!(decoded.is_empty());
    }

    #[test]
    fn test_single_string() {
        let strs = vec!["hello".to_string()];
        let encoded = encode(strs.clone());
        let decoded = decode(&encoded);
        assert_eq!(decoded, strs);
    }

    #[test]
    fn test_empty_string() {
        let strs = vec!["".to_string()];
        let encoded = encode(strs.clone());
        let decoded = decode(&encoded);
        assert_eq!(decoded, strs);
    }

    #[test]
    fn test_multiple_empty_strings() {
        let strs = vec!["".to_string(), "".to_string(), "".to_string()];
        let encoded = encode(strs.clone());
        let decoded = decode(&encoded);
        assert_eq!(decoded, strs);
    }

    #[test]
    fn test_strings_with_colons() {
        let strs = vec!["a:b".to_string(), "1:2:3".to_string()];
        let encoded = encode(strs.clone());
        let decoded = decode(&encoded);
        assert_eq!(decoded, strs);
    }

    #[test]
    fn test_strings_with_numbers() {
        let strs = vec!["123".to_string(), "456:789".to_string()];
        let encoded = encode(strs.clone());
        let decoded = decode(&encoded);
        assert_eq!(decoded, strs);
    }

    #[test]
    fn test_unicode_strings() {
        let strs = vec!["你好".to_string(), "世界".to_string()];
        let encoded = encode(strs.clone());
        let decoded = decode(&encoded);
        assert_eq!(decoded, strs);
    }

    #[test]
    fn test_mixed_empty_and_non_empty() {
        let strs = vec!["".to_string(), "a".to_string(), "".to_string()];
        let encoded = encode(strs.clone());
        let decoded = decode(&encoded);
        assert_eq!(decoded, strs);
    }

    #[test]
    fn test_long_string() {
        let long_string = "a".repeat(10000);
        let strs = vec![long_string.clone()];
        let encoded = encode(strs.clone());
        let decoded = decode(&encoded);
        assert_eq!(decoded, strs);
    }

    #[test]
    fn test_many_small_strings() {
        let strs: Vec<String> = (0..100).map(|i| i.to_string()).collect();
        let encoded = encode(strs.clone());
        let decoded = decode(&encoded);
        assert_eq!(decoded, strs);
    }

    #[test]
    fn test_special_characters() {
        let strs = vec![
            "hello\nworld".to_string(),
            "tab\there".to_string(),
            "null\x00byte".to_string(),
        ];
        let encoded = encode(strs.clone());
        let decoded = decode(&encoded);
        assert_eq!(decoded, strs);
    }

    #[test]
    fn test_v2_basic() {
        let strs = vec!["hello".to_string(), "world".to_string()];
        let encoded = encode_v2(strs.clone());
        let decoded = decode_v2(&encoded);
        assert_eq!(decoded, strs);
    }

    #[test]
    fn test_v2_empty() {
        let strs: Vec<String> = vec![];
        let encoded = encode_v2(strs);
        let decoded = decode_v2(&encoded);
        assert!(decoded.is_empty());
    }

    #[test]
    fn test_consistency_v1_v2() {
        let strs = vec![
            "lint".to_string(),
            "code".to_string(),
            "love".to_string(),
            "you".to_string(),
        ];
        let encoded1 = encode(strs.clone());
        let encoded2 = encode_v2(strs.clone());
        let decoded1 = decode(&encoded1);
        let decoded2 = decode_v2(&encoded2);
        assert_eq!(decoded1, strs);
        assert_eq!(decoded2, strs);
        assert_eq!(decoded1, decoded2);
    }

    #[test]
    fn test_whitespace_strings() {
        let strs = vec![
            " ".to_string(),
            "  ".to_string(),
            "\t".to_string(),
            "\n".to_string(),
        ];
        let encoded = encode(strs.clone());
        let decoded = decode(&encoded);
        assert_eq!(decoded, strs);
    }

    #[test]
    fn test_single_char_strings() {
        let strs: Vec<String> = (0..26).map(|i| ((('a' as u8) + i) as char).to_string()).collect();
        let encoded = encode(strs.clone());
        let decoded = decode(&encoded);
        assert_eq!(decoded, strs);
    }

    #[test]
    fn test_ascii_art() {
        let strs = vec!["┌─┐".to_string(), "│ │".to_string(), "└─┘".to_string()];
        let encoded = encode(strs.clone());
        let decoded = decode(&encoded);
        assert_eq!(decoded, strs);
    }

    #[test]
    fn test_empty_string_at_end() {
        let strs = vec!["a".to_string(), "b".to_string(), "".to_string()];
        let encoded = encode(strs.clone());
        let decoded = decode(&encoded);
        assert_eq!(decoded, strs);
    }

    #[test]
    fn test_empty_string_at_start() {
        let strs = vec!["".to_string(), "a".to_string(), "b".to_string()];
        let encoded = encode(strs.clone());
        let decoded = decode(&encoded);
        assert_eq!(decoded, strs);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("Encode Decode Strings exercises - run tests with cargo test");
}