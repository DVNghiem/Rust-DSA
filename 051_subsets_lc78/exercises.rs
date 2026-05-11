/// Subsets - LeetCode 78
/// Generate all possible subsets of a set of unique integers.

/// Approach 1: Iterative Building
/// Start with empty subset, for each element, add it to all existing subsets.
pub fn subsets_iterative(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![vec![]];
    for num in nums {
        let new_subsets: Vec<Vec<i32>> = result
            .iter()
            .map(|subset| {
                let mut new = subset.clone();
                new.push(num);
                new
            })
            .collect();
        result.extend(new_subsets);
    }
    result
}

/// Approach 2: Bit Manipulation
/// Use bitmask to represent subset membership. For n elements, there are 2^n subsets.
pub fn subsets_bit(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let n = nums.len();
    let total = 1 << n;
    let mut result = Vec::new();

    for mask in 0..total {
        let mut subset = Vec::new();
        for i in 0..n {
            if mask & (1 << i) != 0 {
                subset.push(nums[i]);
            }
        }
        result.push(subset);
    }
    result
}

/// Approach 3: Backtracking
/// Build subsets recursively by including/excluding each element.
pub fn subsets_backtrack(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut path = Vec::new();

    fn backtrack(start: usize, nums: &Vec<i32>, path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        result.push(path.clone());
        for i in start..nums.len() {
            path.push(nums[i]);
            backtrack(i + 1, nums, path, result);
            path.pop();
        }
    }

    backtrack(0, &nums, &mut path, &mut result);
    result
}

/// Wrapper function for tests
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    subsets_iterative(nums)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subsets_basic() {
        let result = subsets(vec![1, 2]);
        assert!(result.contains(&vec![]));
        assert!(result.contains(&vec![1]));
        assert!(result.contains(&vec![2]));
        assert!(result.contains(&vec![1, 2]));
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_subsets_three_elements() {
        let result = subsets(vec![1, 2, 3]);
        assert_eq!(result.len(), 8);
        assert!(result.contains(&vec![]));
        assert!(result.contains(&vec![1]));
        assert!(result.contains(&vec![2]));
        assert!(result.contains(&vec![3]));
        assert!(result.contains(&vec![1, 2]));
        assert!(result.contains(&vec![1, 3]));
        assert!(result.contains(&vec![2, 3]));
        assert!(result.contains(&vec![1, 2, 3]));
    }

    #[test]
    fn test_subsets_single_element() {
        let result = subsets(vec![1]);
        assert_eq!(result.len(), 2);
        assert!(result.contains(&vec![]));
        assert!(result.contains(&vec![1]));
    }

    #[test]
    fn test_subsets_empty() {
        let result = subsets(vec![]);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], vec![]);
    }

    #[test]
    fn test_subsets_large() {
        let nums: Vec<i32> = (1..=10).collect();
        let result = subsets(nums);
        assert_eq!(result.len(), 1024);
    }

    #[test]
    fn test_subsets_bit_basic() {
        let result = subsets_bit(vec![1, 2]);
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_subsets_bit_three() {
        let result = subsets_bit(vec![1, 2, 3]);
        assert_eq!(result.len(), 8);
    }

    #[test]
    fn test_subsets_backtrack_basic() {
        let result = subsets_backtrack(vec![1, 2]);
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_subsets_backtrack_three() {
        let result = subsets_backtrack(vec![1, 2, 3]);
        assert_eq!(result.len(), 8);
    }

    #[test]
    fn test_subsets_iterative_basic() {
        let result = subsets_iterative(vec![1, 2]);
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_subsets_iterative_three() {
        let result = subsets_iterative(vec![1, 2, 3]);
        assert_eq!(result.len(), 8);
    }

    #[test]
    fn test_all_approaches_same_result() {
        let nums = vec![1, 2, 3];
        let iter = subsets_iterative(nums.clone());
        let bit = subsets_bit(nums.clone());
        let back = subsets_backtrack(nums.clone());

        assert_eq!(iter.len(), 8);
        assert_eq!(bit.len(), 8);
        assert_eq!(back.len(), 8);
    }

    #[test]
    fn test_subsets_negative_numbers() {
        let result = subsets(vec![-1, -2]);
        assert_eq!(result.len(), 4);
        assert!(result.contains(&vec![]));
        assert!(result.contains(&vec![-1]));
        assert!(result.contains(&vec![-2]));
        assert!(result.contains(&vec![-1, -2]));
    }

    #[test]
    fn test_subsets_mixed_numbers() {
        let result = subsets(vec![-1, 0, 1]);
        assert_eq!(result.len(), 8);
    }

    #[test]
    fn test_subsets_zeros() {
        let result = subsets(vec![0, 0]);
        // Note: This problem has unique elements, so no duplicates
        // But we test with unique zeros
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_subsets_large_numbers() {
        let result = subsets(vec![1000000, 2000000]);
        assert_eq!(result.len(), 4);
        assert!(result.contains(&vec![1000000, 2000000]));
    }

    #[test]
    fn test_subsets_four_elements() {
        let result = subsets(vec![1, 2, 3, 4]);
        assert_eq!(result.len(), 16);
    }

    #[test]
    fn test_subsets_five_elements() {
        let result = subsets(vec![1, 2, 3, 4, 5]);
        assert_eq!(result.len(), 32);
    }

    #[test]
    fn test_bitmask_correctness_small() {
        let nums = vec![1, 2];
        let result = subsets_bit(nums);
        let mut sorted_result: Vec<Vec<i32>> = result;
        sorted_result.sort();
        let expected: Vec<Vec<i32>> = vec![vec![], vec![1], vec![2], vec![1, 2]];
        assert_eq!(sorted_result, expected);
    }

    #[test]
    fn test_backtrack_includes_empty_set() {
        let result = subsets_backtrack(vec![1, 2, 3]);
        assert!(result.iter().any(|s| s.is_empty()));
    }

    #[test]
    fn test_backtrack_includes_full_set() {
        let result = subsets_backtrack(vec![1, 2, 3]);
        assert!(result.iter().any(|s| s.len() == 3));
    }

    #[test]
    fn test_correct_subset_count_formula() {
        // For n elements, there should be exactly 2^n subsets
        for n in 0..=10 {
            let nums: Vec<i32> = (0..n).collect();
            let result = subsets(nums);
            assert_eq!(result.len(), 1 << n);
        }
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("051_subsets_lc78 exercises - run tests with cargo test");
}
