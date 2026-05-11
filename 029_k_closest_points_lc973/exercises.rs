use std::collections::BinaryHeap;

/// Point struct for clarity
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    dist_sq: i64,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        let dist_sq = (x as i64) * (x as i64) + (y as i64) * (y as i64);
        Point { x, y, dist_sq }
    }

    fn from_vec(v: &[i32]) -> Option<Self> {
        if v.len() >= 2 {
            Some(Point::new(v[0], v[1]))
        } else {
            None
        }
    }
}

/// Approach 1: Sort by distance squared (O(n log n))
///
/// Sort all points by distance to origin, then take first k.
pub fn k_closest_sort(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let mut sorted = points;
    sorted.sort_by_key(|p| {
        let x = p[0] as i64;
        let y = p[1] as i64;
        x * x + y * y
    });
    sorted.into_iter().take(k as usize).collect()
}

/// Approach 2: Max-Heap maintaining k closest points (O(n log k))
///
/// Use a max-heap to keep track of k closest points.
/// Largest element in heap = farthest among our k closest.
/// When we find a closer point, replace the farthest.
pub fn k_closest_heap(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    // Max-heap: largest distance at top
    // Store as (negative dist, x, y) for min-heap behavior with max-heap
    let mut heap: BinaryHeap<(i64, i32, i32)> = BinaryHeap::new();

    for point in points {
        if point.len() < 2 {
            continue;
        }
        let x = point[0];
        let y = point[1];
        let dist_sq = (x as i64) * (x as i64) + (y as i64) * (y as i64);

        heap.push((dist_sq, x, y));

        // Keep only k points - pop farthest (largest dist)
        if heap.len() > k as usize {
            heap.pop();
        }
    }

    // Extract remaining k points
    heap.into_iter()
        .map(|(_, x, y)| vec![x, y])
        .collect()
}

/// Approach 3: Manual max-heap with reverse ordering
pub fn k_closest_max_heap(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    #[derive(Clone)]
    struct HeapItem {
        dist_sq: i64,
        x: i32,
        y: i32,
    }

    impl PartialOrd for HeapItem {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for HeapItem {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            // Reverse: larger dist_sq should come first (max-heap behavior)
            other.dist_sq.cmp(&self.dist_sq)
        }
    }

    impl PartialEq for HeapItem {
        fn eq(&self, other: &Self) -> bool {
            self.dist_sq == other.dist_sq
        }
    }

    impl Eq for HeapItem {}

    let mut heap: BinaryHeap<HeapItem> = BinaryHeap::new();

    for point in points {
        if point.len() < 2 {
            continue;
        }
        let x = point[0];
        let y = point[1];
        let dist_sq = (x as i64) * (x as i64) + (y as i64) * (y as i64);

        heap.push(HeapItem { dist_sq, x, y });

        if heap.len() > k as usize {
            heap.pop();
        }
    }

    heap.into_iter().map(|item| vec![item.x, item.y]).collect()
}

/// Calculate Euclidean distance from origin (for testing)
pub fn distance_squared(x: i32, y: i32) -> i64 {
    (x as i64) * (x as i64) + (y as i64) * (y as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vec_to_point(v: &[i32]) -> (i32, i32) {
        (v[0], v[1])
    }

    fn check_result(result: &[Vec<i32>], expected: &[(i32, i32)], k: i32) {
        assert_eq!(result.len(), k as usize, "Should return exactly k points");
        for r in result {
            assert_eq!(r.len(), 2, "Each point should have x and y");
        }
    }

    #[test]
    fn test_basic_k_closest() {
        // [[1,3],[-2,2]], k=1
        // Distance: (1,3)=10, (-2,2)=8
        // Closest: (-2,2)
        let points = vec![vec![1, 3], vec![-2, 2]];
        let result = k_closest_heap(vec![1, 3], 1);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_k_equals_two() {
        // [[3,3],[5,-1],[-2,4]], k=2
        // Distances: (3,3)=18, (5,-1)=26, (-2,4)=20
        // Closest 2: (3,3) and (-2,4)
        let points = vec![vec![3, 3], vec![5, -1], vec![-2, 4]];
        let result = k_closest_heap(points, 2);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_k_equals_one() {
        let points = vec![vec![1, 0], vec![0, 1], vec![-1, 0], vec![0, -1]];
        let result = k_closest_heap(points, 1);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_k_equals_all() {
        let points = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
        let result = k_closest_heap(points, 3);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_negative_coordinates() {
        let points = vec![vec![-3, -4], vec![0, 0], vec![2, 2]];
        let result = k_closest_heap(points, 2);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_points_at_origin() {
        let points = vec![vec![0, 0], vec![1, 1], vec![0, 0]];
        let result = k_closest_heap(points, 1);
        assert_eq!(result.len(), 1);
        // Origin should be closest (distance = 0)
    }

    #[test]
    fn test_sort_same_as_heap() {
        let points = vec![
            vec![1, 0], vec![0, 1], vec![-2, 2], vec![2, -2], vec![0, -1]
        ];
        let k = 3;
        let sort_result = k_closest_sort(points.clone(), k);
        let heap_result = k_closest_heap(points.clone(), k);
        assert_eq!(sort_result.len(), heap_result.len());
    }

    #[test]
    fn test_large_k() {
        let points = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]];
        let result = k_closest_heap(points, 4);
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_single_point() {
        let points = vec![vec![5, 5]];
        let result = k_closest_heap(points, 1);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], vec![5, 5]);
    }

    #[test]
    fn test_duplicate_points() {
        let points = vec![vec![1, 1], vec![1, 1], vec![0, 0]];
        let result = k_closest_heap(points, 2);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_symmetric_points() {
        // (3,4) and (-3,-4) have same distance from origin
        let points = vec![vec![3, 4], vec![-3, -4], vec![0, 0]];
        let result = k_closest_heap(points, 2);
        assert_eq!(result.len(), 2);
        // Origin must be one of them (distance 0)
        let has_origin = result.iter().any(|p| p[0] == 0 && p[1] == 0);
        assert!(has_origin);
    }

    #[test]
    fn test_large_coordinates() {
        // Using i64 for distance calculation prevents overflow
        let points = vec![vec![1000000, 1000000], vec![-1000000, 1000000], vec![0, 0]];
        let result = k_closest_heap(points, 2);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_max_heap_method() {
        let points = vec![vec![1, 0], vec![0, 1], vec![-1, 0], vec![0, -1]];
        let result = k_closest_max_heap(points, 2);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_max_heap_consistency() {
        let points = vec![
            vec![3, 4], vec![-3, -4], vec![0, 5], vec![1, 0], vec![2, 2]
        ];
        let heap_result = k_closest_heap(points.clone(), 3);
        let max_heap_result = k_closest_max_heap(points, 3);
        assert_eq!(heap_result.len(), max_heap_result.len());
    }

    #[test]
    fn test_empty_input() {
        let points: Vec<Vec<i32>> = vec![];
        let result = k_closest_heap(points, 0);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_distance_calculation() {
        assert_eq!(distance_squared(3, 4), 25); // 3-4-5 triangle
        assert_eq!(distance_squared(0, 0), 0);   // Origin
        assert_eq!(distance_squared(-3, -4), 25); // Negative coordinates
    }

    #[test]
    fn test_all_same_distance() {
        // Points forming a circle around origin
        let points = vec![
            vec![1, 0], vec![0, 1], vec![-1, 0], vec![0, -1]
        ];
        let result = k_closest_heap(points, 4);
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_sort_method_basic() {
        let points = vec![vec![1, 3], vec![-2, 2]];
        let result = k_closest_sort(points, 1);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_sort_preserves_order() {
        let points = vec![vec![3, 3], vec![5, -1], vec![-2, 4]];
        let result = k_closest_sort(points, 2);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_quarter_plane_points() {
        // All points in first quadrant
        let points = vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4]];
        let result = k_closest_heap(points, 2);
        assert_eq!(result.len(), 2);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("K Closest Points exercises - run tests with cargo test");
}