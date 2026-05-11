# Longest Increasing Subsequence - Solution Analysis

## Problem Overview

Find length of longest strictly increasing subsequence in array.

## Solution 1: Binary Search O(n log n)

### Code Implementation

```rust
pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    if nums.is_empty() { return 0; }

    let mut tails = Vec::new();

    for num in nums {
        let pos = match tails.binary_search(&num) {
            Ok(p) => p,
            Err(p) => p,
        };

        if pos == tails.len() {
            tails.push(num);
        } else {
            tails[pos] = num;
        }
    }

    tails.len() as i32
}
```

### Line-by-Line Analysis

1. **`if nums.is_empty() { return 0; }`**: Edge case.

2. **`let mut tails = Vec::new();`**: `tails[i]` = smallest tail value for LIS of length i+1.

3. **`for num in nums { ... }`**: Process each number.

4. **`let pos = match tails.binary_search(&num) { Ok(p) => p, Err(p) => p };`**: Find insertion position. This is O(log n).

5. **`if pos == tails.len() { tails.push(num); } else { tails[pos] = num; }`**: If larger than all tails, append. Otherwise replace first larger element.

### Why This Works

```
For LIS, we want the smallest possible tail for each length.
When we replace tails[pos], we maintain this property.
This allows future elements to fit into longer subsequences.

Example: [10, 9, 2, 5, 3, 7, 101, 18]

tails after each num:
10: [10]
9:  [9] (replace 10)
2:  [2] (replace 9)
5:  [2, 5] (append since 5 > 2)
3:  [2, 3] (replace 5)
7:  [2, 3, 7] (append)
101: [2, 3, 7, 101] (append)
18:  [2, 3, 7, 18] (replace 101)

Length = 4 ✓
```

## Solution 2: DP O(n²)

```rust
pub fn length_of_lis_dp(nums: Vec<i32>) -> i32 {
    if nums.is_empty() { return 0; }

    let n = nums.len();
    let mut dp = vec![1i32; n];
    let mut max_len = 1;

    for i in 1..n {
        for j in 0..i {
            if nums[j] < nums[i] {
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
        max_len = max_len.max(dp[i]);
    }

    max_len
}
```

dp[i] = max length of LIS ending at i.

## Complexity Comparison

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Binary Search | O(n log n) | O(n) | Optimal |
| DP O(n²) | O(n²) | O(n) | Simpler |

## Key Insights

1. **Patience sorting**: The tails array is exactly the piles in patience sorting game.

2. **Binary search position**: Finding where to put each element is O(log n).

3. **Replace vs append**: Append if larger than all (extends LIS). Replace first larger (keeps smallest tail).

## Test Case Analysis

### Test: `test_basic`

```
nums = [10, 9, 2, 5, 3, 7, 101, 18]

Step-by-step:
10 → [10]
9 → [9]
2 → [2]
5 → [2, 5]
3 → [2, 3]
7 → [2, 3, 7]
101 → [2, 3, 7, 101]
18 → [2, 3, 7, 18]

Length = 4 ✓
One LIS: [2, 3, 7, 18]
```

## Follow-up Answers

**Q: Why binary search works?**
A: `tails` is always sorted. Finding position tells us where this element fits in potential LIS.

**Q: Why replace first larger?**
A: Smaller tail for same length means more flexibility for future elements.

**Q: How to get actual subsequence?**
A: Track predecessor indices alongside the binary search, then reconstruct.