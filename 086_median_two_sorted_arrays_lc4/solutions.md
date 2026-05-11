# Solutions Analysis: Median of Two Sorted Arrays (LeetCode 4)

## Solution Overview

We implement three approaches to find the median of two sorted arrays:
1. **Merge** - O(m+n) time, O(m+n) space
2. **Two pointers** - O(m+n) time, O(1) space
3. **Binary search** - O(log m) time, O(1) space (optimal)

## Solution 1: Merge (Simple)

### Algorithm

```rust
pub fn find_median_merge(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let m = nums1.len();
    let n = nums2.len();
    let total = m + n;

    let mut merged = Vec::with_capacity(total);
    let mut i = 0;
    let mut j = 0;

    // Merge like merge sort
    while i < m && j < n {
        if nums1[i] <= nums2[j] {
            merged.push(nums1[i]);
            i += 1;
        } else {
            merged.push(nums2[j]);
            j += 1;
        }
    }

    // Add remaining elements
    while i < m { merged.push(nums1[i]); i += 1; }
    while j < n { merged.push(nums2[j]); j += 1; }

    // Return median
    if total % 2 == 0 {
        (merged[total / 2 - 1] as f64 + merged[total / 2] as f64) / 2.0
    } else {
        merged[total / 2] as f64
    }
}
```

### Trace

```
nums1 = [1, 3], nums2 = [2]
merged = []
i=0, j=0: 1 <= 2 → push 1, i=1
i=1, j=0: 3 > 2 → push 2, j=1
i=1, j=1: i>=m → push 3
merged = [1, 2, 3]
total = 3 (odd)
median = merged[1] = 2 ✓
```

## Solution 2: Two Pointers (Space-Optimized)

### Key Insight

Instead of storing all merged elements, just track the two middle elements as we traverse.

```rust
pub fn find_median_two_pointers(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let total = m + n;
    let mut left = 0;
    let mut right = 0;
    let mut i = 0;
    let mut j = 0;

    // Iterate (total + 1) / 2 times to get left and right
    for _ in 0..(total + 1) / 2 {
        left = right;
        if i < m && (j >= n || nums1[i] <= nums2[j]) {
            right = nums1[i];
            i += 1;
        } else {
            right = nums2[j];
            j += 1;
        }
    }

    if total % 2 == 0 {
        (left + right) as f64 / 2.0
    } else {
        right as f64
    }
}
```

### Trace

```
nums1 = [1, 3], nums2 = [2]
total = 3
(total + 1) / 2 = 2

Iteration 1:
left = 0, right = 0
i < m && (j >= n || nums1[i] <= nums2[j])
  = 0 < 2 && (0 >= 1 || 1 <= 2) = true
right = nums1[0] = 1, i = 1

Iteration 2:
left = 1, right = 1
i < m && (j >= n || nums1[i] <= nums2[j])
  = 1 < 2 && (0 >= 1 || 3 <= 2) = false
right = nums2[0] = 2, j = 1

Result: total is odd, return right = 2 ✓
```

## Solution 3: Binary Search (Optimal)

### Key Insight

The partition-based approach finds the median by partitioning both arrays such that:
- Elements on left side are ≤ elements on right side
- Left side has exactly half the elements

### Algorithm

```rust
pub fn find_median_binary_search(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (m, n) = (nums1.len(), nums2.len());

    // Ensure nums1 is smaller for simpler binary search
    if m > n {
        return find_median_binary_search(nums2, nums1);
    }

    let mut left = 0;
    let mut right = m;

    while left <= right {
        let partition_x = (left + right) / 2;
        let partition_y = (m + n + 1) / 2 - partition_x;

        // Get boundary values
        let max_left_x = if partition_x == 0 { i32::MIN } else { nums1[partition_x - 1] };
        let min_right_x = if partition_x == m { i32::MAX } else { nums1[partition_x] };

        let max_left_y = if partition_y == 0 { i32::MIN } else { nums2[partition_y - 1] };
        let min_right_y = if partition_y == n { i32::MAX } else { nums2[partition_y] };

        // Check if partition is correct
        if max_left_x <= min_right_y && max_left_y <= min_right_x {
            // Found! Calculate median
            if (m + n) % 2 == 0 {
                let left_max = max_left_x.max(max_left_y);
                let right_min = min_right_x.min(min_right_y);
                return (left_max as f64 + right_min as f64) / 2.0;
            } else {
                return max_left_x.max(max_left_y) as f64;
            }
        } else if max_left_x > min_right_y {
            right = partition_x - 1;  // Too far right in X, go left
        } else {
            left = partition_x + 1;    // Too far left in X, go right
        }
    }

    0.0
}
```

### Trace

```
nums1 = [1, 3], m = 2
nums2 = [2], n = 1
total = 3

Ensure m <= n: m=2 > n=1, so swap
nums1 = [2], nums2 = [1, 3]
m = 1, n = 2

left = 0, right = 1

Iteration 1:
partition_x = (0 + 1) / 2 = 0
partition_y = (1 + 2 + 1) / 2 - 0 = 2

max_left_x = i32::MIN (partition_x == 0)
min_right_x = nums1[0] = 2 (partition_x == m)
max_left_y = i32::MIN (partition_y == 0)
min_right_y = nums2[1] = 3 (partition_y = 2)

Check: i32::MIN <= 3 && i32::MIN <= 2 → true
(m+n) % 2 = 1 → odd
Return max_left_x.max(max_left_y) = i32::MIN.max(i32::MIN) = i32::MIN? 

Wait, that's wrong. Let me check the conditions again...

Actually partition_y = (m + n + 1) / 2 - partition_x
                 = (1 + 2 + 1) / 2 - 0
                 = 4 / 2 - 0
                 = 2

But n = 2, so valid partition_y values are 0, 1, 2.

For partition_y = 2:
max_left_y = nums2[1] = 3
min_right_y = i32::MAX (partition_y == n, n=2)

max_left_y = 3, min_right_x = 2
3 <= 2? No! So this partition is wrong.

Our condition: max_left_y <= min_right_x
3 <= 2? No, so we need to move left (partition_x is too large)
```

Wait, let me recompute more carefully.

After swap, we have:
nums1 = [2] (smaller, m=1)
nums2 = [1, 3] (larger, n=2)

total = 3

We want partition_x + partition_y = (3+1)/2 = 2 (left side has 2 elements)

Iteration 1:
partition_x = (0+1)/2 = 0
partition_y = 2 - 0 = 2

Boundary values:
max_left_x = i32::MIN (partition_x == 0)
min_right_x = nums1[0] = 2 (partition_x == m)
max_left_y = nums2[1] = 3 (partition_y == 2, so use index 1)
min_right_y = i32::MAX (partition_y == n)

Conditions:
max_left_x <= min_right_y: -INF <= INF → true
max_left_y <= min_right_x: 3 <= 2 → false

Since max_left_y > min_right_x, partition_x is too small, go right.
left = 0 → left = 1

Iteration 2:
partition_x = (1+1)/2 = 1
partition_y = 2 - 1 = 1

Boundary values:
max_left_x = nums1[0] = 2 (partition_x = 1, so use index 0)
min_right_x = i32::MAX (partition_x == m)
max_left_y = nums2[0] = 1 (partition_y = 1, use index 0)
min_right_y = nums2[1] = 3 (partition_y = 1 < n, use index 1)

Conditions:
max_left_x <= min_right_y: 2 <= 3 → true
max_left_y <= min_right_x: 1 <= INF → true

Both conditions satisfied! Found partition.

(m+n) % 2 = 1 (odd)
Return max_left_x.max(max_left_y) = max(2, 1) = 2 ✓

## Complexity Comparison

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Merge | O(m+n) | O(m+n) | Simple but not optimal |
| Two pointers | O(m+n) | O(1) | Good but still linear |
| Binary search | O(log m) | O(1) | Optimal solution |

The binary search is optimal because each iteration halves the search space.

## Edge Cases

### Empty arrays
```rust
nums1 = [], nums2 = [1] → median = 1
```

### One element each
```rust
nums1 = [1], nums2 = [2] → median = 1.5
```

### Different sizes
```rust
nums1 = [1, 2], nums2 = [3, 4, 5, 6]
combined sorted: [1, 2, 3, 4, 5, 6]
median positions 2 and 3 (0-indexed): values 3 and 4
median = (3+4)/2 = 3.5
```

## Why Binary Search is Correct

### Invariant

We maintain that:
- All elements in left partition are ≤ all elements in right partition
- Number of elements in left = (m+n+1)/2

When we find the correct partition, we know:
- Left max = max(max_left_x, max_left_y)
- Right min = min(min_right_x, min_right_y)

If total is odd, median = left max
If total is even, median = (left max + right min) / 2

## Conclusion

The binary search approach is optimal:
1. O(log m) time complexity
2. O(1) space complexity
3. Divides search space in half each iteration
4. Correctly handles all edge cases