/// Approach: Sort by start time, then merge overlapping
pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if intervals.is_empty() {
        return vec![];
    }

    let mut intervals = intervals;
    intervals.sort_by_key(|v| v[0]);

    let mut result = vec![intervals[0].clone()];

    for i in 1..intervals.len() {
        let last = result.last_mut().unwrap();
        if intervals[i][0] <= last[1] {
            // Overlapping: extend the end if needed
            last[1] = last[1].max(intervals[i][1]);
        } else {
            // No overlap: add new interval
            result.push(intervals[i].clone());
        }
    }

    result
}

/// Check if two intervals overlap
fn is_overlap(a: &[i32], b: &[i32]) -> bool {
    a[0] <= b[1] && b[0] <= a[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let intervals = vec![
            vec![1, 3],
            vec![2, 6],
            vec![8, 10],
            vec![15, 18],
        ];
        let result = merge(intervals);
        assert_eq!(result, vec![vec![1, 6], vec![8, 10], vec![15, 18]]);
    }

    #[test]
    fn test_empty() {
        let intervals: Vec<Vec<i32>> = vec![];
        assert!(merge(intervals).is_empty());
    }

    #[test]
    fn test_single() {
        let intervals = vec![vec![1, 5]];
        assert_eq!(merge(intervals), vec![vec![1, 5]]);
    }

    #[test]
    fn test_no_overlap() {
        let intervals = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        let result = merge(intervals);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_all_overlap() {
        let intervals = vec![vec![1, 10], vec![2, 3], vec![4, 5], vec![6, 7]];
        assert_eq!(merge(intervals), vec![vec![1, 10]]);
    }

    #[test]
    fn test_adjacent_intervals() {
        // [1,2] and [2,3] are considered overlapping (share endpoint)
        let intervals = vec![vec![1, 2], vec![2, 3], vec![3, 4]];
        let result = merge(intervals);
        assert_eq!(result, vec![vec![1, 4]]);
    }

    #[test]
    fn test_exactly_touching() {
        let intervals = vec![vec![1, 5], vec![5, 10]];
        let result = merge(intervals);
        // [1,5] and [5,10] should merge to [1,10]
        assert_eq!(result, vec![vec![1, 10]]);
    }

    #[test]
    fn test_unsorted_input() {
        let intervals = vec![vec![8, 10], vec![1, 3], vec![15, 18], vec![2, 6]];
        let result = merge(intervals);
        assert_eq!(result, vec![vec![1, 6], vec![8, 10], vec![15, 18]]);
    }

    #[test]
    fn test_negative_numbers() {
        let intervals = vec![vec![-5, -3], vec![-2, 1], vec![0, 5]];
        let result = merge(intervals);
        assert_eq!(result, vec![vec![-5, 1], vec![0, 5]]);
    }

    #[test]
    fn test_same_start() {
        let intervals = vec![vec![1, 4], vec![1, 4], vec![1, 4]];
        let result = merge(intervals);
        assert_eq!(result, vec![vec![1, 4]]);
    }

    #[test]
    fn test_contained_intervals() {
        let intervals = vec![vec![1, 10], vec![2, 3], vec![4, 5], vec![6, 7]];
        assert_eq!(merge(intervals), vec![vec![1, 10]]);
    }

    #[test]
    fn test_large_gap() {
        let intervals = vec![vec![1, 2], vec![100, 101]];
        let result = merge(intervals);
        assert_eq!(result.len(), 2);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("048_merge_intervals_lc56 exercises - run tests with cargo test");
}
