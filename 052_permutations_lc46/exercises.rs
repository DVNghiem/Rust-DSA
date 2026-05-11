/// Permutations - LeetCode 46
/// Generate all possible permutations of an array of distinct integers.

/// Approach 1: Backtracking with used array
/// Track which elements have been used to avoid repetition.
pub fn permutations_backtrack(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut path = Vec::new();
    let mut used = vec![false; nums.len()];

    fn backtrack(nums: &Vec<i32>, path: &mut Vec<i32>, used: &mut Vec<bool>, result: &mut Vec<Vec<i32>>) {
        if path.len() == nums.len() {
            result.push(path.clone());
            return;
        }
        for i in 0..nums.len() {
            if !used[i] {
                used[i] = true;
                path.push(nums[i]);
                backtrack(nums, path, used, result);
                path.pop();
                used[i] = false;
            }
        }
    }

    backtrack(&nums, &mut path, &mut used, &mut result);
    result
}

/// Approach 2: Swap-based backtracking (in-place)
/// Swap elements to generate permutations without extra space for used array.
pub fn permutations_swap(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut nums = nums;

    fn backtrack(nums: &mut Vec<i32>, start: usize, result: &mut Vec<Vec<i32>>) {
        if start == nums.len() {
            result.push(nums.clone());
            return;
        }
        for i in start..nums.len() {
            nums.swap(start, i);
            backtrack(nums, start + 1, result);
            nums.swap(start, i); // backtrack (undo swap)
        }
    }

    backtrack(&mut nums, 0, &mut result);
    result
}

/// Wrapper function for tests
pub fn permutations(nums: Vec<i32>) -> Vec<Vec<i32>> {
    permutations_backtrack(nums)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutations_basic() {
        let result = permutations(vec![1, 2]);
        assert_eq!(result.len(), 2);
        assert!(result.contains(&vec![1, 2]));
        assert!(result.contains(&vec![2, 1]));
    }

    #[test]
    fn test_permutations_three() {
        let result = permutations(vec![1, 2, 3]);
        assert_eq!(result.len(), 6);
    }

    #[test]
    fn test_permutations_single() {
        let result = permutations(vec![1]);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], vec![1]);
    }

    #[test]
    fn test_permutations_empty() {
        let result = permutations(vec![]);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], vec![]);
    }

    #[test]
    fn test_permutations_swap_basic() {
        let result = permutations_swap(vec![1, 2]);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_permutations_swap_three() {
        let result = permutations_swap(vec![1, 2, 3]);
        assert_eq!(result.len(), 6);
    }

    #[test]
    fn test_permutations_backtrack_basic() {
        let result = permutations_backtrack(vec![1, 2]);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_permutations_backtrack_three() {
        let result = permutations_backtrack(vec![1, 2, 3]);
        assert_eq!(result.len(), 6);
    }

    #[test]
    fn test_all_approaches_same_count() {
        let nums = vec![1, 2, 3];
        assert_eq!(permutations_backtrack(nums.clone()).len(), 6);
        assert_eq!(permutations_swap(nums).len(), 6);
    }

    #[test]
    fn test_permutations_contains_all_elements() {
        let result = permutations(vec![1, 2, 3]);
        for perm in result {
            assert!(perm.contains(&1));
            assert!(perm.contains(&2));
            assert!(perm.contains(&3));
            assert_eq!(perm.len(), 3);
        }
    }

    #[test]
    fn test_permutations_no_duplicates() {
        let result = permutations(vec![1, 2, 3]);
        let mut seen: Vec<Vec<i32>> = Vec::new();
        for perm in result {
            assert!(!seen.contains(&perm));
            seen.push(perm);
        }
    }

    #[test]
    fn test_factorial_count() {
        // n! permutations for n elements
        assert_eq!(permutations(vec![1]).len(), 1);
        assert_eq!(permutations(vec![1, 2]).len(), 2);
        assert_eq!(permutations(vec![1, 2, 3]).len(), 6);
        assert_eq!(permutations(vec![1, 2, 3, 4]).len(), 24);
    }

    #[test]
    fn test_permutations_negative() {
        let result = permutations(vec![-1, 1]);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_permutations_zeros() {
        let result = permutations(vec![0, 0]);
        // This problem is for distinct integers
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_permutations_large_numbers() {
        let result = permutations(vec![100, 200, 300]);
        assert_eq!(result.len(), 6);
        for perm in result {
            assert!(perm.contains(&100));
            assert!(perm.contains(&200));
            assert!(perm.contains(&300));
        }
    }

    #[test]
    fn test_permutations_order_matters() {
        let result = permutations(vec![1, 2, 3]);
        let mut seen = std::collections::HashSet::new();
        for perm in &result {
            let key = perm.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
            assert!(!seen.contains(&key), "Duplicate permutation found");
            seen.insert(key);
        }
    }

    #[test]
    fn test_swap_preserves_elements() {
        let result = permutations_swap(vec![1, 2, 3]);
        for perm in result {
            assert_eq!(perm.iter().sum::<i32>(), 6);
        }
    }

    #[test]
    fn test_backtrack_preserves_elements() {
        let result = permutations_backtrack(vec![1, 2, 3]);
        for perm in result {
            assert_eq!(perm.iter().product::<i32>(), 6);
        }
    }

    #[test]
    fn test_empty_input_produces_empty_permutation() {
        let result = permutations(vec![]);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 0);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("052_permutations_lc46 exercises - run tests with cargo test");
}
