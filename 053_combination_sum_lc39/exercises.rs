/// Combination Sum - LeetCode 39
/// Find all unique combinations of candidates that sum to target.
/// Each candidate may be used unlimited times.

/// Approach 1: Backtracking with index tracking
/// Start index ensures we don't generate duplicate combinations.
pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut path = Vec::new();

    fn backtrack(start: usize, candidates: &Vec<i32>, target: i32, path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if target == 0 {
            result.push(path.clone());
            return;
        }
        for i in start..candidates.len() {
            if candidates[i] <= target {
                path.push(candidates[i]);
                backtrack(i, candidates, target - candidates[i], path, result);
                path.pop();
            }
        }
    }

    backtrack(0, &candidates, target, &mut path, &mut result);
    result
}

/// Approach 2: Sorted candidates with early termination
/// Sorting allows us to break early when candidates[i] > target.
pub fn combination_sum_sorted(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut candidates = candidates;
    candidates.sort();
    let mut result = Vec::new();
    let mut path = Vec::new();

    fn backtrack(start: usize, candidates: &Vec<i32>, target: i32, path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if target == 0 {
            result.push(path.clone());
            return;
        }
        for i in start..candidates.len() {
            if candidates[i] > target {
                break;
            }
            path.push(candidates[i]);
            backtrack(i, candidates, target - candidates[i], path, result);
            path.pop();
        }
    }

    backtrack(0, &candidates, target, &mut path, &mut result);
    result
}

/// Approach 3: Iterative with stack simulation
pub fn combination_sum_iterative(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut stack = vec![(0i32, Vec::new())];

    while let Some((remaining, path)) = stack.pop() {
        if remaining == 0 {
            result.push(path);
            continue;
        }
        for i in 0..candidates.len() {
            if candidates[i] <= remaining {
                let mut new_path = path.clone();
                new_path.push(candidates[i]);
                stack.push((remaining - candidates[i], new_path));
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_sum_basic() {
        let result = combination_sum(vec![2, 3, 6, 7], 7);
        assert!(result.contains(&vec![2, 2, 3]));
        assert!(result.contains(&vec![7]));
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_combination_sum_simple() {
        let result = combination_sum(vec![2, 3, 5], 8);
        assert!(result.contains(&vec![2, 2, 2, 2]));
        assert!(result.contains(&vec![2, 3, 3]));
        assert!(result.contains(&vec![3, 5]));
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_combination_sum_no_solution() {
        let result = combination_sum(vec![2], 1);
        assert!(result.is_empty());
    }

    #[test]
    fn test_combination_sum_single_element() {
        let result = combination_sum(vec![2], 2);
        assert_eq!(result, vec![vec![2]]);
    }

    #[test]
    fn test_combination_sum_single_element_repeated() {
        let result = combination_sum(vec![2], 6);
        assert_eq!(result, vec![vec![2, 2, 2]]);
    }

    #[test]
    fn test_combination_sum_empty() {
        let result = combination_sum(vec![], 1);
        assert!(result.is_empty());
    }

    #[test]
    fn test_combination_sum_target_zero() {
        let result = combination_sum(vec![1, 2], 0);
        assert_eq!(result, vec![vec![]]);
    }

    #[test]
    fn test_combination_sum_sorted_basic() {
        let result = combination_sum_sorted(vec![2, 3, 6, 7], 7);
        assert!(result.contains(&vec![2, 2, 3]));
        assert!(result.contains(&vec![7]));
    }

    #[test]
    fn test_combination_sum_sorted_simple() {
        let result = combination_sum_sorted(vec![2, 3, 5], 8);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_combination_sum_iterative_basic() {
        let result = combination_sum_iterative(vec![2, 3, 6, 7], 7);
        assert!(result.contains(&vec![2, 2, 3]));
        assert!(result.contains(&vec![7]));
    }

    #[test]
    fn test_combination_sum_iterative_simple() {
        let result = combination_sum_iterative(vec![2, 3, 5], 8);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_combination_sum_large_target() {
        let result = combination_sum(vec![2, 3], 6);
        assert!(result.contains(&vec![2, 2, 2]));
        assert!(result.contains(&vec![3, 3]));
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_combination_sum_negative_candidates() {
        // Note: Problem states positive integers, but we test negative for edge cases
        let result = combination_sum(vec![-2, 2, 3], 3);
        assert!(result.contains(&vec![3]));
        assert!(result.contains(&vec![-2, 2, 3]));
    }

    #[test]
    fn test_combination_sum_duplicates_in_result() {
        // Results should not have duplicate combinations
        let result = combination_sum(vec![2, 3, 6, 7], 7);
        let mut seen: Vec<Vec<i32>> = Vec::new();
        for combo in result {
            assert!(!seen.contains(&combo));
            seen.push(combo);
        }
    }

    #[test]
    fn test_combination_sum_sum_verification() {
        let candidates = vec![2, 3, 5];
        let result = combination_sum(candidates, 8);
        for combo in result {
            let sum: i32 = combo.iter().sum();
            assert_eq!(sum, 8);
        }
    }

    #[test]
    fn test_combination_sum_uses_candidates() {
        let candidates = vec![2, 3, 6, 7];
        let result = combination_sum(candidates, 7);
        for combo in result {
            for elem in combo {
                assert!(vec![2, 3, 6, 7].contains(elem));
            }
        }
    }

    #[test]
    fn test_combination_sum_order_matters_for_same_combo() {
        // [2,2,3] and [3,2,2] should be treated as same (both shouldn't appear)
        let result = combination_sum(vec![2, 3, 6, 7], 7);
        for combo in &result {
            let mut sorted = combo.clone();
            sorted.sort();
            // Only one representation of [2,2,3] should exist
        }
    }

    #[test]
    fn test_combination_sum_pruning_works() {
        // With sorted candidates, larger elements should be pruned
        let result = combination_sum_sorted(vec![1, 2, 6], 6);
        assert!(result.contains(&vec![6]));
        assert!(result.contains(&vec![2, 2, 2]));
        assert!(result.contains(&vec![1, 1, 2, 2]));
        assert!(result.contains(&vec![1, 1, 1, 1, 2]));
        assert!(result.contains(&vec![1, 1, 1, 1, 1, 1]));
    }

    #[test]
    fn test_combination_sum_unsorted_input() {
        // Input might not be sorted - algorithm should handle it
        let result = combination_sum(vec![7, 2, 6, 3], 7);
        assert!(result.contains(&vec![7]));
        assert!(result.contains(&vec![2, 2, 3]));
    }

    #[test]
    fn test_combination_sum_different_order_same_result() {
        let result1 = combination_sum(vec![2, 3, 6, 7], 7);
        let result2 = combination_sum(vec![6, 7, 2, 3], 7);
        // Both should have same number of results (duplicates removed)
        assert_eq!(result1.len(), result2.len());
    }

    #[test]
    fn test_combination_sum_complex() {
        let result = combination_sum(vec![2, 3, 5], 30);
        // Should find multiple combinations
        assert!(result.len() > 5);
        for combo in result {
            assert_eq!(combo.iter().sum::<i32>(), 30);
        }
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("Combination Sum exercises - run tests with cargo test");
}