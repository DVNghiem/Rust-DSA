# Three Sum (LeetCode #15)

## Problem Statement

Given an integer array `nums`, return all the triplets `[nums[i], nums[j], nums[k]]` such that:
- `i != j`, `i != k`, `j != k`
- `nums[i] + nums[j] + nums[k] == 0`

The solution set must not contain duplicate triplets.

## Examples

```
Input: nums = [-1, 0, 1, 2, -1, -4]
Output: [[-1, -1, 2], [-1, 0, 1]]
Explanation: Triplets that sum to zero.
```

## Approaches Overview

### Approach 1: Sort + Two Pointers O(n²)
Sort, then for each element use two pointers on remaining subarray.

```rust
pub fn three_sum(nums: &[i32]) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut nums = nums.to_vec();
    nums.sort();

    for i in 0..nums.len().saturating_sub(2) {
        if i > 0 && nums[i] == nums[i - 1] { continue; }

        let mut left = i + 1;
        let mut right = nums.len() - 1;

        while left < right {
            let sum = nums[i] + nums[left] + nums[right];
            match sum.cmp(&0) {
                std::cmp::Ordering::Less => left += 1,
                std::cmp::Ordering::Greater => right -= 1,
                std::cmp::Ordering::Equal => {
                    result.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;
                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }
                }
            }
        }
    }
    result
}
```

### Why Sort First?

1. **Enables two pointers**: We need a sorted array for the two-pointer technique
2. **Enables deduplication**: Same values become adjacent, easy to skip
3. **Establishes ordering**: Negative numbers are grouped together

## Visual Walkthrough

```
nums = [-1, 0, 1, 2, -1, -4] → sorted = [-4, -1, -1, 0, 1, 2]

Step 1: i=0, nums[i]=-4
  left=1, right=5: sum=-4+(-1)+2=-3 < 0 → left++ (left=2)
  left=2, right=5: sum=-4+(-1)+2=-3 < 0 → left++ (left=3)
  left=3, right=5: sum=-4+0+2=-2 < 0 → left++ (left=4)
  left=4, right=5: sum=-4+1+2=-1 < 0 → left++ (left=5)
  left == right, stop

Step 2: i=1, nums[i]=-1 (skip if same as i-1, but i=1, i-1=0, different)
  left=2, right=5: sum=-1+(-1)+2=0 → Add [-1,-1,2], left=3
  left=3, right=5: sum=-1+0+2=1 > 0 → right-- (right=4)
  left=3, right=4: sum=-1+0+1=0 → Add [-1,0,1], left=4
  left=4, right=4, stop

Result: [[-1,-1,2], [-1,0,1]]
```

## Complexity Analysis Table

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Brute Force | O(n³) | O(1) | Check all triplets |
| Sort + Two Pointers | O(n²) | O(n) | Sorting + search |

## Edge Cases to Consider

1. **Empty array**: Return empty vector
2. **Less than 3 elements**: Return empty vector
3. **All same elements**: nums = [0,0,0,0] → [[0,0,0]]
4. **No triplet sums to zero**: Return empty vector
5. **Duplicate values**: Skip to avoid duplicate triplets

## Key Insight: Deduplication

Without deduplication: `[[-1,0,1], [-1,0,1], [-1,0,1]]` with same values
With deduplication: Skip when `nums[i] == nums[i-1]` at start of loop

This ensures each unique triplet appears exactly once.

## Related Problems

### LeetCode 16: 3Sum Closest
Find triplet closest to target sum.

### LeetCode 18: 4Sum
Find all quadruplets that sum to target.

### LeetCode 454: 4Sum II
Four arrays, find quadruplets across all arrays.

## Exercises

### Exercise 1: Basic Three Sum
Implement sort + two pointers solution.

### Exercise 2: Three Sum Closest
Find triplet with sum closest to target.

### Exercise 3: Four Sum
Find all unique quadruplets.

### Exercise 4: Count Triplets with Sum
Count triplets that sum to zero (not return them).

### Exercise 5: Three Sum Smaller
Count triplets with sum less than target.

## Key Takeaways

1. **Sort first** enables two-pointer technique
2. **Skip duplicates** to avoid duplicate triplets
3. **Fix one element**, use two pointers for the rest
4. **Time complexity O(n²)** is optimal for this problem
5. **Handle negative numbers** correctly in sum calculation

## Real-World Applications

1. **Portfolio balancing**: Find investments that balance to zero
2. **Data analysis**: Find feature combinations summing to target
3. **Game theory**: Find winning moves that sum to zero
4. **Scheduling**: Find shift combinations totaling hours
5. **Cryptography**: Key sum problems in secret sharing
