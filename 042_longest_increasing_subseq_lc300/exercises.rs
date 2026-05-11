/// Approach 1: Binary Search (Optimal - O(n log n))
///
/// Maintain "tails" array where tails[i] = smallest tail value
/// for LIS of length i+1.
/// For each num, binary search to find position.
/// Replace if found, append if not.
pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut tails = Vec::new();

    for num in nums {
        // Binary search for num in tails
        let pos = match tails.binary_search(&num) {
            Ok(p) => p,
            Err(p) => p,
        };

        if pos == tails.len() {
            tails.push(num);
        } else {
            tails[pos] = num;
        }
    }

    tails.len() as i32
}

/// Approach 2: DP O(n²) - simpler but slower
pub fn length_of_lis_dp(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let n = nums.len();
    let mut dp = vec![1i32; n];
    let mut max_len = 1;

    for i in 1..n {
        for j in 0..i {
            if nums[j] < nums[i] {
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
        max_len = max_len.max(dp[i]);
    }

    max_len
}

/// Approach 3: With actual subsequence reconstruction
pub fn length_of_lis_with_sequence(nums: Vec<i32>) -> (i32, Vec<i32>) {
    if nums.is_empty() {
        return (0, vec![]);
    }

    let n = nums.len();
    let mut dp = vec![1i32; n];
    let mut parent = vec![-1isize; n];

    // Find LIS using DP
    let mut max_len = 1;
    let mut max_idx = 0;

    for i in 1..n {
        for j in 0..i {
            if nums[j] < nums[i] && dp[j] + 1 > dp[i] {
                dp[i] = dp[j] + 1;
                parent[i] = j as isize;
            }
        }
        if dp[i] > max_len as i32 {
            max_len = dp[i] as usize;
            max_idx = i;
        }
    }

    // Reconstruct sequence
    let mut lis = Vec::new();
    let mut idx = max_idx as isize;
    while idx >= 0 {
        lis.push(nums[idx as usize]);
        idx = parent[idx as usize];
    }
    lis.reverse();

    (max_len as i32, lis)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
        assert_eq!(length_of_lis(nums), 4);
    }

    #[test]
    fn test_empty() {
        let nums: Vec<i32> = vec![];
        assert_eq!(length_of_lis(nums), 0);
    }

    #[test]
    fn test_single() {
        assert_eq!(length_of_lis(vec![1]), 1);
    }

    #[test]
    fn test_decreasing() {
        // [5, 4, 3, 2, 1] - only LIS of length 1
        let nums = vec![5, 4, 3, 2, 1];
        assert_eq!(length_of_lis(nums), 1);
    }

    #[test]
    fn test_all_same() {
        // [1, 1, 1, 1] - strictly increasing requires different
        let nums = vec![1, 1, 1, 1];
        assert_eq!(length_of_lis(nums), 1);
    }

    #[test]
    fn test_already_sorted() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(length_of_lis(nums), 5);
    }

    #[test]
    fn test_binary_search_matches_dp() {
        let test_cases = vec![
            vec![10, 9, 2, 5, 3, 7, 101, 18],
            vec![1, 2, 3, 4, 5],
            vec![5, 4, 3, 2, 1],
            vec![1],
            vec![],
            vec![1, 1, 1, 1],
        ];

        for nums in test_cases {
            let bs = length_of_lis(nums.clone());
            let dp = length_of_lis_dp(nums);
            assert_eq!(bs, dp, "Failed for {:?}", nums);
        }
    }

    #[test]
    fn test_long_sequence() {
        let nums: Vec<i32> = (0..1000).rev().collect();
        assert_eq!(length_of_lis(nums), 1);
    }

    #[test]
    fn test_random_order() {
        let nums = vec![3, 1, 5, 2, 4];
        // LIS could be [1, 2, 4] or [1, 5] or [3, 5] etc.
        // Longest is length 3
        assert_eq!(length_of_lis(nums), 3);
    }

    #[test]
    fn test_with_sequence_reconstruction() {
        let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
        let (len, seq) = length_of_lis_with_sequence(nums);
        assert_eq!(len, 4);
        // Verify it's increasing
        for i in 1..seq.len() {
            assert!(seq[i] > seq[i - 1]);
        }
        // Verify it's a subsequence
        let mut idx = 0;
        for num in &nums {
            if idx < seq.len() && *num == seq[idx] {
                idx += 1;
            }
        }
        assert_eq!(idx, seq.len());
    }

    #[test]
    fn test_two_elements() {
        assert_eq!(length_of_lis(vec![1, 2]), 2);
        assert_eq!(length_of_lis(vec![2, 1]), 1);
    }

    #[test]
    fn test_negative_numbers() {
        let nums = vec![-1, -2, -3, -4];
        assert_eq!(length_of_lis(nums), 4);
    }

    #[test]
    fn test_mixed_numbers() {
        let nums = vec![5, 1, 6, 2, 7, 8, 3];
        // LIS is [1, 2, 7, 8] or [5, 6, 7, 8] etc.
        assert_eq!(length_of_lis(nums), 4);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("Longest Increasing Subsequence exercises - run tests with cargo test");
}