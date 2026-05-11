/// Queue Reconstruction by Height - LeetCode 406
/// Reconstruct queue based on height and visibility count.

use std::collections::VecDeque;

/// Approach: Sort by height (desc), then insert by k (asc)
/// Taller people first, insert at position k.
pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut people = people;
    // Sort: height descending, then k ascending
    people.sort_by(|a, b| {
        if a[0] != b[0] {
            b[0].cmp(&a[0]) // taller first
        } else {
            a[1].cmp(&b[1]) // smaller k first
        }
    });

    let mut result: Vec<Vec<i32>> = Vec::new();
    for p in &people {
        let k = p[1] as usize;
        result.insert(k, p.clone());
    }

    result
}

/// Alternative: Use VecDeque for O(n) insert at front
pub fn reconstruct_queue_deque(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut people = people;
    people.sort_by(|a, b| {
        if a[0] != b[0] {
            b[0].cmp(&a[0])
        } else {
            a[1].cmp(&b[1])
        }
    });

    let mut result: VecDeque<Vec<i32>> = VecDeque::new();
    for p in people {
        let k = p[1] as usize;
        if k >= result.len() {
            result.push_back(p);
        } else {
            result.insert(k, p);
        }
    }

    result.into_iter().collect()
}

/// Verification function
pub fn verify_queue(queue: &Vec<Vec<i32>>) -> bool {
    for (i, person) in queue.iter().enumerate() {
        let h = person[0];
        let k = person[1] as usize;
        let mut count = 0;
        for j in 0..queue.len() {
            if queue[j][0] >= h {
                count += 1;
            }
            if count > k + 1 { return false; }
        }
        if count != k + 1 { return false; }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reconstruct_basic() {
        let result = reconstruct_queue(vec![
            vec![7,0], vec![4,4], vec![7,1], vec![5,0], vec![6,1], vec![5,2]
        ]);
        assert_eq!(result.len(), 6);
        assert!(verify_queue(&result));
    }

    #[test]
    fn test_reconstruct_simple() {
        let result = reconstruct_queue(vec![vec![7,0], vec![7,1]]);
        assert_eq!(result.len(), 2);
        assert!(verify_queue(&result));
    }

    #[test]
    fn test_reconstruct_single() {
        let result = reconstruct_queue(vec![vec![5,0]]);
        assert_eq!(result.len(), 1);
        assert!(verify_queue(&result));
    }

    #[test]
    fn test_reconstruct_empty() {
        let result = reconstruct_queue(vec![]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_reconstruct_two_same_height() {
        let result = reconstruct_queue(vec![vec![5,0], vec![5,2]]);
        assert!(verify_queue(&result));
    }

    #[test]
    fn test_reconstruct_three_same_height() {
        let result = reconstruct_queue(vec![vec![5,0], vec![5,1], vec![5,2]]);
        assert!(verify_queue(&result));
    }

    #[test]
    fn test_reconstruct_all_different_heights() {
        let result = reconstruct_queue(vec![vec![1,0], vec![2,0], vec![3,0]]);
        assert!(verify_queue(&result));
    }

    #[test]
    fn test_reconstruct_descending_k() {
        let result = reconstruct_queue(vec![vec![6,0], vec![5,0], vec![4,0], vec![3,0], vec![2,0], vec![1,0]]);
        assert!(verify_queue(&result));
    }

    #[test]
    fn test_reconstruct_ascending_k() {
        let result = reconstruct_queue(vec![vec![1,0], vec![2,1], vec![3,2], vec![4,3], vec![5,4]]);
        assert!(verify_queue(&result));
    }

    #[test]
    fn test_reconstruct_mixed() {
        let result = reconstruct_queue(vec![vec![1,0], vec![2,0], vec![2,1], vec![3,1]]);
        assert!(verify_queue(&result));
    }

    #[test]
    fn test_reconstruct_deque_basic() {
        let result = reconstruct_queue_deque(vec![
            vec![7,0], vec![4,4], vec![7,1], vec![5,0], vec![6,1], vec![5,2]
        ]);
        assert_eq!(result.len(), 6);
        assert!(verify_queue(&result));
    }

    #[test]
    fn test_reconstruct_deque_simple() {
        let result = reconstruct_queue_deque(vec![vec![7,0], vec![7,1]]);
        assert!(verify_queue(&result));
    }

    #[test]
    fn test_reconstruct_same_result_as_deque() {
        let input = vec![vec![7,0], vec![4,4], vec![7,1], vec![5,0], vec![6,1], vec![5,2]];
        assert_eq!(reconstruct_queue(input.clone()), reconstruct_queue_deque(input));
    }

    #[test]
    fn test_verify_queue_valid() {
        let queue = vec![vec![5,0], vec![7,0], vec![5,2], vec![6,1], vec![7,1], vec![4,4]];
        assert!(verify_queue(&queue));
    }

    #[test]
    fn test_verify_queue_invalid() {
        let queue = vec![vec![5,0], vec![5,2], vec![6,1]];
        assert!(!verify_queue(&queue));
    }

    #[test]
    fn test_reconstruct_large_k() {
        let result = reconstruct_queue(vec![vec![1,0], vec![0,1]]);
        assert!(verify_queue(&result));
    }

    #[test]
    fn test_reconstruct_all_same_height() {
        let result = reconstruct_queue(vec![vec![5,0], vec![5,1], vec![5,2], vec![5,3]]);
        assert!(verify_queue(&result));
    }

    #[test]
    fn test_reconstruct_k_equals_count() {
        let result = reconstruct_queue(vec![vec![5,0], vec![4,0], vec![3,0]]);
        assert!(verify_queue(&result));
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("Queue Reconstruction exercises - run tests with cargo test");
}