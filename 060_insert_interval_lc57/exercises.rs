/// Insert Interval - LeetCode 57
/// Insert a new interval and merge all overlapping intervals.

/// Approach: Three-phase insertion
/// 1. Add intervals before new interval (no overlap)
/// 2. Merge intervals that overlap with new
/// 3. Add remaining intervals after new
pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut i = 0;
    let n = intervals.len();
    let (new_start, new_end) = (new_interval[0], new_interval[1]);

    // Phase 1: Add intervals before new_interval (completely before)
    while i < n && intervals[i][1] < new_start {
        result.push(intervals[i].clone());
        i += 1;
    }

    // Phase 2: Merge all overlapping intervals
    let mut start = new_start;
    let mut end = new_end;
    while i < n && intervals[i][0] <= end {
        start = start.min(intervals[i][0]);
        end = end.max(intervals[i][1]);
        i += 1;
    }
    result.push(vec![start, end]);

    // Phase 3: Add remaining intervals
    while i < n {
        result.push(intervals[i].clone());
        i += 1;
    }

    result
}

/// Alternative: Single pass with merge tracking
pub fn insert_single_pass(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let (new_start, new_end) = (new_interval[0], new_interval[1]);
    let mut merged_end = new_end;
    let mut merged_start = new_start;
    let mut in_new = false;

    for interval in intervals {
        if interval[1] < new_start || interval[0] > new_end {
            if in_new {
                result.push(vec![merged_start, merged_end]);
                in_new = false;
            }
            result.push(interval);
        } else {
            if !in_new {
                merged_start = new_start.min(interval[0]);
                merged_end = new_end.max(interval[1]);
                in_new = true;
            } else {
                merged_start = merged_start.min(interval[0]);
                merged_end = merged_end.max(interval[1]);
            }
        }
    }

    if in_new {
        result.push(vec![merged_start, merged_end]);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_basic() {
        assert_eq!(insert(vec![vec![1,3],vec![6,9]], vec![2,5]), vec![vec![1,9]]);
    }

    #[test]
    fn test_insert_no_overlap_before() {
        assert_eq!(insert(vec![vec![1,2],vec![3,4]], vec![5,6]), vec![vec![1,2],vec![3,4],vec![5,6]]);
    }

    #[test]
    fn test_insert_no_overlap_after() {
        assert_eq!(insert(vec![vec![5,6]], vec![1,2]), vec![vec![1,2],vec![5,6]]);
    }

    #[test]
    fn test_insert_overlap_multiple() {
        let result = insert(
            vec![vec![1,2],[3,5],[6,7],[8,10],[12,16]],
            vec![4,8]
        );
        assert_eq!(result, vec![vec![1,2],vec![3,10],vec![12,16]]);
    }

    #[test]
    fn test_insert_before_all() {
        assert_eq!(insert(vec![vec![5,6],vec![7,8]], vec![1,2]), vec![vec![1,2],vec![5,6],vec![7,8]]);
    }

    #[test]
    fn test_insert_after_all() {
        assert_eq!(insert(vec![vec![1,2]], vec![5,6]), vec![vec![1,2],vec![5,6]]);
    }

    #[test]
    fn test_insert_empty() {
        assert_eq!(insert(vec![], vec![5,6]), vec![vec![5,6]]);
    }

    #[test]
    fn test_insert_single() {
        assert_eq!(insert(vec![vec![1,5]], vec![2,3]), vec![vec![1,5]]);
    }

    #[test]
    fn test_insert_adjacent_before() {
        assert_eq!(insert(vec![vec![1,2],[5,6]], vec![3,4]), vec![vec![1,2],vec![3,4],vec![5,6]]);
    }

    #[test]
    fn test_insert_adjacent_after() {
        assert_eq!(insert(vec![vec![1,2],[3,4]], vec![5,6]), vec![vec![1,2],vec![3,4],vec![5,6]]);
    }

    #[test]
    fn test_insert_contains() {
        assert_eq!(insert(vec![vec![1,5]], vec![2,4]), vec![vec![1,5]]);
    }

    #[test]
    fn test_insert_contained() {
        assert_eq!(insert(vec![vec![2,4]], vec![1,5]), vec![vec![1,5]]);
    }

    #[test]
    fn test_insert_single_pass_basic() {
        let result = insert_single_pass(vec![vec![1,3],vec![6,9]], vec![2,5]);
        assert_eq!(result, vec![vec![1,9]]);
    }

    #[test]
    fn test_insert_single_pass_overlap() {
        let result = insert_single_pass(
            vec![vec![1,2],[3,5],[6,7],[8,10],[12,16]],
            vec![4,8]
        );
        assert_eq!(result, vec![vec![1,2],vec![3,10],vec![12,16]]);
    }

    #[test]
    fn test_insert_same_result() {
        let input = vec![vec![1,3],vec![6,9]];
        let new = vec![2,5];
        assert_eq!(insert(input.clone(), new.clone()),
                   insert_single_pass(input, new));
    }

    #[test]
    fn test_insert_overlap_front() {
        assert_eq!(insert(vec![vec![2,5]], vec![1,3]), vec![vec![1,5]]);
    }

    #[test]
    fn test_insert_overlap_back() {
        assert_eq!(insert(vec![vec![1,3]], vec![2,5]), vec![vec![1,5]]);
    }

    #[test]
    fn test_insert_complex() {
        let result = insert(
            vec![vec![1,3],[6,9],[10,12]],
            vec![4,7]
        );
        assert_eq!(result, vec![vec![1,3],vec![4,9],vec![10,12]]);
    }

    #[test]
    fn test_insert_large_intervals() {
        let result = insert(
            vec![vec![1,100]],
            vec![50,150]
        );
        assert_eq!(result, vec![vec![1,150]]);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("Insert Interval exercises - run tests with cargo test");
}