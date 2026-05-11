/// Basic Calculator - LeetCode 224
/// Evaluate expression with +, -, and parentheses.

/// Approach: Stack-based evaluation
/// Push result and sign when encountering '('
/// Pop and combine when encountering ')'
pub fn calculate(s: String) -> i32 {
    let bytes = s.as_bytes();
    let mut result = 0;
    let mut sign = 1;
    let mut stack: Vec<i32> = Vec::new();

    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'+' => sign = 1,
            b'-' => sign = -1,
            b'(' => {
                // Save current state
                stack.push(result);
                stack.push(sign);
                result = 0;
                sign = 1;
            }
            b')' => {
                // Apply current result with its sign
                result = result * stack.pop().unwrap() + stack.pop().unwrap();
            }
            b' ' => {}
            c => {
                // Parse multi-digit number
                let mut num = (c - b'0') as i32;
                while i + 1 < bytes.len() && bytes[i + 1].is_ascii_digit() {
                    i += 1;
                    num = num * 10 + (bytes[i] - b'0') as i32;
                }
                result += num * sign;
            }
        }
        i += 1;
    }

    result
}

/// Alternative: Recursive parsing
pub fn calculate_recursive(s: String) -> i32 {
    let bytes = s.as_bytes();
    let mut pos = 0;

    fn parse(bytes: &[u8], pos: &mut usize) -> i32 {
        let mut result = 0;
        let mut sign = 1;

        while *pos < bytes.len() {
            match bytes[*pos] {
                b' ' => { *pos += 1; continue; }
                b'+' => { sign = 1; *pos += 1; }
                b'-' => { sign = -1; *pos += 1; }
                b'(' => {
                    *pos += 1;
                    let val = parse(bytes, pos);
                    result += sign * val;
                }
                b')' => { *pos += 1; return result; }
                _ => {
                    let mut num = 0;
                    while *pos < bytes.len() && bytes[*pos].is_ascii_digit() {
                        num = num * 10 + (bytes[*pos] - b'0') as i32;
                        *pos += 1;
                    }
                    result += sign * num;
                    if *pos < bytes.len() && bytes[*pos] == b')' {
                        return result;
                    }
                    continue;
                }
            }
        }
        result
    }

    parse(bytes, &mut pos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_basic() {
        assert_eq!(calculate("1 + 1".to_string()), 2);
    }

    #[test]
    fn test_calculate_subtraction() {
        assert_eq!(calculate("2-1".to_string()), 1);
    }

    #[test]
    fn test_calculate_expression() {
        assert_eq!(calculate("2-1+2".to_string()), 3);
    }

    #[test]
    fn test_calculate_parentheses() {
        assert_eq!(calculate("(1)".to_string()), 1);
    }

    #[test]
    fn test_calculate_complex_paren() {
        assert_eq!(calculate("(1+(4+5+2)-3)+(6+8)".to_string()), 23);
    }

    #[test]
    fn test_calculate_empty() {
        assert_eq!(calculate("".to_string()), 0);
    }

    #[test]
    fn test_calculate_single_number() {
        assert_eq!(calculate("5".to_string()), 5);
    }

    #[test]
    fn test_calculate_two_numbers() {
        assert_eq!(calculate("5+3".to_string()), 8);
    }

    #[test]
    fn test_calculate_negative_result() {
        assert_eq!(calculate("1-5".to_string()), -4);
    }

    #[test]
    fn test_calculate_leading_space() {
        assert_eq!(calculate("  2+3".to_string()), 5);
    }

    #[test]
    fn test_calculate_trailing_space() {
        assert_eq!(calculate("2+3  ".to_string()), 5);
    }

    #[test]
    fn test_calculate_nested_paren() {
        assert_eq!(calculate("((1))".to_string()), 1);
    }

    #[test]
    fn test_calculate_mixed() {
        assert_eq!(calculate("1 + 2 - 3".to_string()), 0);
    }

    #[test]
    fn test_calculate_recursive_basic() {
        assert_eq!(calculate_recursive("1 + 1".to_string()), 2);
    }

    #[test]
    fn test_calculate_recursive_subtraction() {
        assert_eq!(calculate_recursive("2-1".to_string()), 1);
    }

    #[test]
    fn test_calculate_recursive_complex() {
        assert_eq!(calculate_recursive("(1+(4+5+2)-3)+(6+8)".to_string()), 23);
    }

    #[test]
    fn test_calculate_same_as_recursive() {
        let tests = vec![
            "1 + 1",
            "2-1",
            "2-1+2",
            "(1)",
            "(1+(4+5+2)-3)+(6+8)",
            "1",
            "",
            "  2+3  ",
        ];
        for s in tests {
            let s = s.to_string();
            let r1 = calculate(s.clone());
            let r2 = calculate_recursive(s.clone());
            assert_eq!(r1, r2, "Mismatch for '{}'", s);
        }
    }

    #[test]
    fn test_calculate_large_numbers() {
        assert_eq!(calculate("123456789".to_string()), 123456789);
    }

    #[test]
    fn test_calculate_long_expression() {
        assert_eq!(calculate("1+2+3+4+5+6+7+8+9+10".to_string()), 55);
    }

    #[test]
    fn test_calculate_multiple_paren() {
        assert_eq!(calculate("(1+2)+(3+4)".to_string()), 10);
    }

    #[test]
    fn test_calculate_negative_paren() {
        assert_eq!(calculate("-(1+2)".to_string()), -3);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("068_basic_calculator_lc224 exercises - run tests with cargo test");
}
