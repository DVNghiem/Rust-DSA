//! Exercises for First Missing Positive (LeetCode 41)
//!
//! # Topics Covered
//! - In-place array modification
//! - Index as hash
//! - O(n) time, O(1) space algorithm
//! - Smallest missing positive
//!
//! # Difficulty: Hard

/// In-place marking approach: O(n) time, O(1) space
pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let n = nums.len() as i32;

    for i in 0..n as usize {
        while nums[i] > 0 && nums[i] <= n && nums[(nums[i] - 1) as usize] != nums[i] {
            let idx = (nums[i] - 1) as usize;
            nums.swap(i, idx);
        }
    }

    for i in 0..n as usize {
        if nums[i] != (i + 1) as i32 {
            return (i + 1) as i32;
        }
    }
    n + 1
}

/// Alternative using HashSet - O(n) time, O(n) space
pub fn first_missing_positive_hash(nums: Vec<i32>) -> i32 {
    let n = nums.len() as i32;
    let mut seen = std::collections::HashSet::new();

    for num in &nums {
        if *num > 0 && *num <= n {
            seen.insert(*num);
        }
    }

    for i in 1..=n {
        if !seen.contains(&i) {
            return i;
        }
    }
    n + 1
}

/// Sort-based approach - O(n log n) time, O(1) space (ignoring sort)
pub fn first_missing_positive_sort(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort();
    let n = nums.len() as i32;

    let mut expected = 1;
    for num in nums {
        if num == expected {
            expected += 1;
        } else if num > expected {
            return expected;
        }
    }
    expected
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_example_1() {
        assert_eq!(first_missing_positive(vec![1, 2, 0]), 3);
    }

    #[test]
    fn test_basic_example_2() {
        assert_eq!(first_missing_positive(vec![3, 4, -1, 1]), 2);
    }

    #[test]
    fn test_basic_example_3() {
        assert_eq!(first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
    }

    #[test]
    fn test_empty_array() {
        assert_eq!(first_missing_positive(vec![]), 1);
    }

    #[test]
    fn test_single_element_1() {
        assert_eq!(first_missing_positive(vec![1]), 2);
    }

    #[test]
    fn test_single_element_missing() {
        assert_eq!(first_missing_positive(vec![2]), 1);
    }

    #[test]
    fn test_already_has_1() {
        assert_eq!(first_missing_positive(vec![1, 2, 3]), 4);
    }

    #[test]
    fn test_all_negative() {
        assert_eq!(first_missing_positive(vec![-1, -2, -3]), 1);
    }

    #[test]
    fn test_all_zeros() {
        assert_eq!(first_missing_positive(vec![0, 0, 0]), 1);
    }

    #[test]
    fn test_mixed_positive_negative() {
        assert_eq!(first_missing_positive(vec![-5, -3, 0, 1, 2, 4]), 3);
    }

    #[test]
    fn test_sequential_1_to_n() {
        assert_eq!(first_missing_positive(vec![1, 2, 3, 4, 5]), 6);
    }

    #[test]
    fn test_sequential_0_to_n() {
        assert_eq!(first_missing_positive(vec![0, 1, 2, 3, 4]), 5);
    }

    #[test]
    fn test_duplicate_1() {
        assert_eq!(first_missing_positive(vec![1, 1, 1, 1]), 2);
    }

    #[test]
    fn test_duplicate_missing() {
        assert_eq!(first_missing_positive(vec![2, 2, 2]), 1);
    }

    #[test]
    fn test_large_array_sequential() {
        let nums: Vec<i32> = (1..100).collect();
        assert_eq!(first_missing_positive(nums), 100);
    }

    #[test]
    fn test_missing_at_end() {
        let mut nums: Vec<i32> = (1..50).collect();
        nums.push(51);
        // Missing 50
        assert_eq!(first_missing_positive(nums), 50);
    }

    #[test]
    fn test_missing_at_start() {
        let nums: Vec<i32> = vec![2, 3, 4, 5];
        assert_eq!(first_missing_positive(nums), 1);
    }

    #[test]
    fn test_hash_agrees_with_inplace() {
        let test_cases = vec![
            vec![1, 2, 0],
            vec![3, 4, -1, 1],
            vec![7, 8, 9, 11, 12],
            vec![],
            vec![1],
            vec![2],
            vec![1, 2, 3],
            vec![-1, -2],
            vec![1, 1, 2, 2, 3, 3],
        ];

        for nums in test_cases {
            let inplace = first_missing_positive(nums.clone());
            let hash = first_missing_positive_hash(nums.clone());
            assert_eq!(inplace, hash, "Failed for {:?}", nums);
        }
    }

    #[test]
    fn test_sort_agrees_with_inplace() {
        let test_cases = vec![
            vec![1, 2, 0],
            vec![3, 4, -1, 1],
            vec![7, 8, 9, 11, 12],
            vec![],
            vec![1],
            vec![2],
            vec![1, 2, 3],
        ];

        for nums in test_cases {
            let inplace = first_missing_positive(nums.clone());
            let sort = first_missing_positive_sort(nums.clone());
            assert_eq!(inplace, sort, "Failed for {:?}", nums);
        }
    }

    #[test]
    fn test_answer_always_between_1_and_n_plus_1() {
        let n = 10;
        let nums: Vec<i32> = (1..=n).collect();
        let result = first_missing_positive(nums);
        assert!(result >= 1 && result <= n + 1);
    }

    #[test]
    fn test_two_elements_missing_1() {
        assert_eq!(first_missing_positive(vec![2, 3]), 1);
    }

    #[test]
    fn test_two_elements_no_missing() {
        assert_eq!(first_missing_positive(vec![1, 2]), 3);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("087_first_missing_positive_lc41 exercises - run tests with cargo test");
}
