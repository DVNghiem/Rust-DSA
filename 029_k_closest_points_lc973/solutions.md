# K Closest Points to Origin - Solution Analysis

## Problem Overview

Given points as [x, y] coordinates and integer k, return k points closest to origin (0, 0). Distance uses Euclidean formula: sqrt(x² + y²).

## Solution 1: Sort by Distance Squared

### Code Implementation

```rust
pub fn k_closest_sort(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let mut sorted = points;
    sorted.sort_by_key(|p| {
        let x = p[0] as i64;
        let y = p[1] as i64;
        x * x + y * y
    });
    sorted.into_iter().take(k as usize).collect()
}
```

### Line-by-Line Analysis

1. **`let mut sorted = points;`**: Clone/create mutable copy to sort.

2. **`sorted.sort_by_key(|p| { ... })`**: Sort by computed key (distance squared).

3. **`let x = p[0] as i64; let y = p[1] as i64;`**: Convert to i64 to prevent overflow. x² + y² can overflow i32 (e.g., x=50000, x²=2.5B > i32::MAX).

4. **`x * x + y * y`**: Distance squared (no need for sqrt - sqrt is monotonic, comparison is same).

5. **`sorted.into_iter().take(k as usize).collect()`**: Take first k elements.

### Why Sort Works

- Sorting puts points in order of increasing distance
- First k elements are exactly the k closest
- O(n log n) is fine for moderate n

## Solution 2: Max-Heap (O(n log k))

### Code Implementation

```rust
pub fn k_closest_heap(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let mut heap: BinaryHeap<(i64, i32, i32)> = BinaryHeap::new();

    for point in points {
        if point.len() < 2 { continue; }
        let x = point[0];
        let y = point[1];
        let dist_sq = (x as i64) * (x as i64) + (y as i64) * (y as i64);

        heap.push((dist_sq, x, y));

        // Keep only k points - pop farthest (largest dist)
        if heap.len() > k as usize {
            heap.pop();
        }
    }

    heap.into_iter().map(|(_, x, y)| vec![x, y]).collect()
}
```

### Line-by-Line Analysis

1. **`let mut heap: BinaryHeap<(i64, i32, i32)> = BinaryHeap::new();`**: Create max-heap storing (distance², x, y) tuples.

2. **`for point in points { ... }`**: Process each point.

3. **`if point.len() < 2 { continue; }`**: Skip invalid points.

4. **`let dist_sq = (x as i64) * (x as i64) + (y as i64) * (y as i64);`**: Compute distance squared with i64 to avoid overflow.

5. **`heap.push((dist_sq, x, y));`**: Add point to heap.

6. **`if heap.len() > k as usize { heap.pop(); }`**: If exceed k, remove largest (farthest). BinaryHeap pops largest by default.

7. **`heap.into_iter().map(|(_, x, y)| vec![x, y]).collect()`**: Extract all k points, ignoring distances.

### Why Max-Heap?

```
Heap always contains our k closest points.
When a new point arrives:
  - If heap has < k: add it (might be one of k closest)
  - If heap has k:
    - If new point is closer than current farthest (top of max-heap):
      → Replace farthest with new point
    - Else: new point is not in k closest, discard it

Result: After processing all points, heap contains exactly k closest.
```

### Max-Heap vs Min-Heap

```
Why use max-heap for "k closest" problem?
- Max-heap gives us the LARGEST distance among our k closest
- When we find something closer, we can eject the farthest
- This keeps heap size bounded at k

Using min-heap would require keeping all n points, then extracting k smallest.
That would be O(n log n) vs O(n log k).
```

## Solution 3: Max-Heap with Custom Struct

```rust
impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist_sq.cmp(&self.dist_sq)  // Reverse for max-heap behavior
    }
}
```

### Line-by-Line Analysis

1. **`#[derive(Clone)] struct HeapItem { ... }`**: Custom struct with dist_sq, x, y.

2. **`impl PartialOrd for HeapItem { ... }`**: Implement custom ordering.

3. **`other.dist_sq.cmp(&self.dist_sq)`**: Reverse comparison - larger dist_sq comes first (max-heap).

4. **`impl Eq for HeapItem {}`**: Required by Ord.

5. **All other logic same as Solution 2**.

## Complexity Comparison

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Sort All | O(n log n) | O(n) | Simple, clean |
| Max-Heap | O(n log k) | O(k) | Better when k << n |
| Quickselect | O(n) avg | O(1) | O(n²) worst case |

## When to Use Which

```
k << n (e.g., k=3, n=10000):
  → Use Max-Heap: O(n log k) = O(10000 * log 3) ≈ O(15000)

k ≈ n (e.g., k=5000, n=10000):
  → Use Sort: O(n log n) = O(10000 * log 10000) ≈ O(130000)
  → Actually similar in this case

k = n:
  → Sort or heap both O(n log n) essentially
  → Sort is simpler
```

## Key Insights

1. **Distance squared is sufficient**: sqrt is monotonic, so comparing d² is equivalent to comparing d. Avoids floating point issues.

2. **i64 for distance**: x² + y² can overflow i32 for large coordinates. Must use i64.

3. **Max-heap for k closest**: Keeps track of k closest efficiently by always ejecting farthest.

4. **Heap size bounded**: When heap size > k, we pop. This ensures O(k) space.

## Test Case Analysis

### Test: `test_basic_k_closest`

```
Input: [[1,3],[-2,2]], k=1

(1,3): dist² = 1 + 9 = 10
(-2,2): dist² = 4 + 4 = 8 ← smaller

Sorted: [(-2,2), (1,3)]
Take first k=1: [(-2,2)] ✓
```

### Test: `test_k_equals_two`

```
Input: [[3,3],[5,-1],[-2,4]], k=2

(3,3): dist² = 9 + 9 = 18
(5,-1): dist² = 25 + 1 = 26
(-2,4): dist² = 4 + 16 = 20

Sorted by dist: [(3,3), (-2,4), (5,-1)]
Take first 2: [(3,3), (-2,4)] ✓
```

### Test: `test_large_coordinates`

```
Input: [[1000000, 1000000], [-1000000, 1000000], [0,0]]

(1000000, 1000000): (10^6)² + (10^6)² = 2*10^12 → exceeds i32!
Using i64: 2_000_000_000_000 ✓

Result correctly identifies (0,0) as closest.
```

## Why Not Quickselect?

```
Quickselect: O(n) average, O(n²) worst case
- Partitions around pivot
- Recursively searches relevant partition

Heap approach: O(n log k)
- Guaranteed O(n log k)
- No pathological worst case
- Simpler to implement correctly
```

## Follow-up Answers

**Q: How to handle 3D points?**
A: Extend to (x, y, z), distance² = x² + y² + z².

**Q: Can you return distances too?**
A: Yes, modify to return (dist_sq, x, y) and compute sqrt when needed.

**Q: Multiple k queries on same data?**
A: Sort once, O(1) per query. Or build spatial index for repeated queries.

**Q: What about integer overflow?**
A: Always use i64 for distance² calculation. x=50000 → x²=2.5B > i32::MAX≈2.1B.