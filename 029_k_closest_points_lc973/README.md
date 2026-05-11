# K Closest Points to Origin - LeetCode 973

## Problem Statement

Given an array of points where `points[i] = [xi, yi]` represents a point on the X-Y plane and an integer k, return the k closest points to the origin `(0, 0)`.

The distance between two points is defined as the Euclidean distance: $\sqrt{(x_2 - x_1)^2 + (y_2 - y_1)^2}$.

You can return the answer in any order.

## Visual Walkthrough

```
Example 1:
Input: points = [[1,3],[-2,2]], k = 1
Output: [[-2,2]]
Explanation: Distance from (-2,2) to origin = sqrt(4+4) = sqrt(8) ≈ 2.83
            Distance from (1,3) to origin = sqrt(1+9) = sqrt(10) ≈ 3.16
            (-2,2) is closer

Example 2:
Input: points = [[3,3],[5,-1],[-2,4]], k = 2
Output: [[3,3],[-2,4]]
Explanation: Distances:
  - (3,3): sqrt(9+9) = sqrt(18) ≈ 4.24
  - (5,-1): sqrt(25+1) = sqrt(26) ≈ 5.10
  - (-2,4): sqrt(4+16) = sqrt(20) ≈ 4.47
  Closest 2: (3,3) and (-2,4)
```

### Euclidean Distance Visualization

```
Point (x, y) to origin (0, 0):

    y
    ^
    |
    |     * (x, y)
    |    /
    |   /
    |  /
    | /
    |/ θ
    +----------→ x
    0

Distance = sqrt(x² + y²)
```

### Sorting vs Heap Approach

```
Input: points = [[1,0],[0,1],[-2,2]], k = 2

Sorted by distance:
1. (0,1): dist² = 0+1 = 1
2. (1,0): dist² = 1+0 = 1
3. (-2,2): dist² = 4+4 = 8

First k = 2: [[0,1],[1,0]] ✓

Max-Heap approach (keep k closest):
- Push all points, maintain heap of size k
- Largest in heap is farthest of our k closest
- When new point is closer, replace the farthest
```

## Approach Comparison

| Approach | Time | Space | Description |
|----------|-------|-------|-------------|
| Sort All | O(n log n) | O(n) | Sort by distance, take first k |
| Max-Heap | O(n log k) | O(k) | Keep k closest, more efficient for small k |
| Quickselect | O(n) avg | O(1) | Partition-based, O(n²) worst |
| Binary Search | O(n log n) | O(n) | Precompute distances, binary search |

### Why Max-Heap for k Closest?

- If k << n, we don't need to sort everything
- Heap of size k lets us efficiently find farthest among our k closest
- When we find a closer point, we remove the farthest (max from heap)
- Time complexity: O(n log k) vs O(n log n)

## Implementation Strategy

### Approach 1: Sort by Distance Squared

```rust
fn k_closest_sort(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let mut sorted = points;
    sorted.sort_by_key(|p| p[0] * p[0] + p[1] * p[1]);
    sorted.into_iter().take(k as usize).collect()
}
```

### Approach 2: Max-Heap

```rust
use std::collections::BinaryHeap;

fn k_closest_heap(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let mut heap: BinaryHeap<(i32, i32, i32)> = BinaryHeap::new();

    for point in points {
        let dist_sq = point[0] * point[0] + point[1] * point[1];
        heap.push((dist_sq, point[0], point[1]));
        if heap.len() > k as usize {
            heap.pop(); // Remove farthest (largest distance)
        }
    }

    // Extract remaining k points
    heap.into_iter().map(|(d, x, y)| vec![x, y]).collect()
}
```

## Edge Cases

1. **k = n**: Return all points
2. **k = 1**: Return single closest point
3. **Points at origin**: Distance = 0
4. **Negative coordinates**: Squaring removes sign issues
5. **Large coordinates**: Use i64 to prevent overflow (x² + y² can overflow i32)
6. **Duplicate distances**: Any order is acceptable

## Test Cases

1. Basic k closest
2. k = 1 (single point)
3. k = all points
4. Points with negative coordinates
5. Points at origin
6. Large coordinate values
7. Multiple points with same distance
8. Large input size

## Solution Explanation

### Distance Calculation

Distance squared = x² + y²

We use squared distance to avoid expensive sqrt() computation. For comparison purposes, sqrt is monotonic, so comparing d² is equivalent to comparing d.

### Max-Heap Logic

Max-Heap stores our k closest points. The "largest" distance in heap is the farthest among our k closest.

For each point:
1. Calculate distance squared
2. Push into heap
3. If heap size > k, pop (remove farthest)
4. After processing all, heap contains k closest

## Complexity Analysis

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Sort | O(n log n) | O(n) | Simple, clean |
| Max-Heap | O(n log k) | O(k) | Better when k << n |

## Follow-up Questions

1. How would you handle 3D points?
2. Can you return exact distances with points?
3. What if you need to query k closest multiple times?