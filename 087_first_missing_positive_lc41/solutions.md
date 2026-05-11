# Solutions Analysis: First Missing Positive (LeetCode 41)

## Problem Overview

Find the smallest missing positive integer in an unsorted array.

## Solution 1: In-Place Marking (Optimal)

### Algorithm

```rust
pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let n = nums.len() as i32;

    for i in 0..n as usize {
        while nums[i] > 0 && nums[i] <= n && nums[(nums[i] - 1) as usize] != nums[i] {
            let idx = (nums[i] - 1) as usize;
            nums.swap(i, idx);
        }
    }

    for i in 0..n as usize {
        if nums[i] != (i + 1) as i32 {
            return (i + 1) as i32;
        }
    }
    n + 1
}
```

### Key Insight

For an array of length n, the answer is always between 1 and n+1. We can use the array itself as a hash table by placing each value v at index v-1 if 1 <= v <= n.

### In-Place Marking Trace

```
nums = [3, 4, -1, 1]
n = 4

i=0: nums[0] = 3
  nums[0] > 0 && nums[0] <= 4 → true
  nums[3] = 1, not equal to 3
  swap(0, 2): nums = [-1, 4, 3, 1]

  i=0 still (due to while loop)
  nums[0] = -1
  -1 <= 0 → false, continue

i=1: nums[1] = 4
  nums[1] > 0 && nums[1] <= 4 → true
  nums[3] = 1, not equal to 4
  swap(1, 3): nums = [-1, 1, 3, 4]

  i=1 still
  nums[1] = 1
  nums[0] = -1, not equal to 1
  swap(1, 0): nums = [1, -1, 3, 4]

  i=1 still
  nums[1] = -1
  -1 <= 0 → false, continue

i=2: nums[2] = 3
  nums[2] > 0 && nums[2] <= 4 → true
  nums[2] = 3, nums[(3-1)]=nums[2]=3 → already correct

i=3: nums[3] = 4
  nums[3] > 0 && nums[3] <= 4 → true
  nums[3] = 4, nums[(4-1)]=nums[3]=4 → already correct

Final nums = [1, -1, 3, 4]

Second pass: find first index where nums[i] != i+1
i=0: nums[0]=1 == 1 ✓
i=1: nums[1]=-1 != 2 → return 2 ✓
```

### Why O(n) Time?

Each element is moved at most once (to its correct position), so total operations = O(n).

### Why O(1) Space?

We modify the array in-place and only use a constant amount of extra variables.

## Solution 2: HashSet

```rust
pub fn first_missing_positive_hash(nums: Vec<i32>) -> i32 {
    let n = nums.len() as i32;
    let mut seen = std::collections::HashSet::new();

    for num in &nums {
        if *num > 0 && *num <= n {
            seen.insert(*num);
        }
    }

    for i in 1..=n {
        if !seen.contains(&i) {
            return i;
        }
    }
    n + 1
}
```

Time: O(n), Space: O(n)

## Solution 3: Sorting

```rust
pub fn first_missing_positive_sort(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort();
    let n = nums.len() as i32;

    let mut expected = 1;
    for num in nums {
        if num == expected {
            expected += 1;
        } else if num > expected {
            return expected;
        }
    }
    expected
}
```

Time: O(n log n), Space: O(1) (or O(n) if we count the sort)

## Edge Cases

### Empty array
```rust
[] → return 1
```

### Single element
```rust
[1] → nums is [1], no missing → return 2
[2] → return 1 (2 > 1, so 1 is missing)
```

### All negative
```rust
[-1, -2, -3] → return 1
```

### Sequential 1 to n
```rust
[1, 2, 3, 4, 5] → return 6 (= n + 1)
```

### Missing in middle
```rust
[3, 4, -1, 1] → return 2
```

## Complexity Comparison

| Approach | Time | Space |
|----------|------|-------|
| In-place | O(n) | O(1) |
| HashSet | O(n) | O(n) |
| Sort | O(n log n) | O(1)* |

*Ignoring the sort's own space

## Why In-Place Works

1. For array of size n, answer ∈ [1, n+1]
2. Each number v where 1 ≤ v ≤ n can be placed at index v-1
3. After placement, the first index i where nums[i] ≠ i+1 means i+1 is missing

## Conclusion

The in-place solution is optimal:
- O(n) time (each element moved at most once)
- O(1) space (no extra data structures)
- Elegant use of array indices as hash positions