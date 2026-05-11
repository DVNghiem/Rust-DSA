# Product of Array Except Self (LeetCode #238)

## Problem Statement

Given an integer array `nums`, return an array `answer` such that `answer[i]` is equal to the product of all the elements of `nums` except `nums[i]`.

The product of any prefix or suffix of `nums` is guaranteed to fit in a 32-bit integer.

You must write an algorithm that runs in **O(n)** time and **without using the division operation**.

## Examples

```
Input: nums = [1, 2, 3, 4]
Output: [24, 12, 8, 6]
Explanation: [2*3*4, 1*3*4, 1*2*4, 1*2*3]

Input: nums = [-1, 1, 0, -3, 3]
Output: [0, 0, 9, 0, 0]
```

## Approaches Overview

### Approach 1: Prefix and Suffix Products O(n) - Preferred
Calculate prefix products going forward, and suffix products going backward.

```rust
pub fn product_except_self(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();
    let mut result = vec![1; n];

    // Calculate prefix products
    let mut prefix = 1;
    for i in 0..n {
        result[i] = prefix;
        prefix *= nums[i];
    }

    // Calculate suffix products and combine
    let mut suffix = 1;
    for i in (0..n).rev() {
        result[i] *= suffix;
        suffix *= nums[i];
    }

    result
}
```

### Approach 2: Total Product with Zero Handling O(n)
Calculate total product and divide by each element (requires division).

```rust
pub fn product_except_self_divide(nums: &[i32]) -> Vec<i32> {
    let total: i64 = nums.iter().map(|&x| x as i64).product();
    let zero_count = nums.iter().filter(|&&x| x == 0).count();

    match zero_count {
        0 => nums.iter().map(|&x| (total / x as i64) as i32).collect(),
        1 => nums.iter().map(|&x| if x == 0 { total as i32 } else { 0 }).collect(),
        _ => vec![0; nums.len()],
    }
}
```

## The Key Insight

For each index i:
```
answer[i] = product of all elements to the left of i * product of all elements to the right of i
```

We can precompute both of these in a single pass.

## Visual Walkthrough: Prefix-Suffix Approach

```
nums = [1, 2, 3, 4]

Step 1: Calculate prefix products (elements to the LEFT of each position)
        nums:   [1,  2,  3,  4]
        prefix: [1,  1,  2,  6]
                  ↑   ↑   ↑
                  │   │   └── 1*2*3 = 6 (product of indices 0,1,2)
                  │   └────── 1*2 = 2 (product of index 0)
                  └────────── 1 (nothing to the left of index 0)

Step 2: Calculate suffix products (elements to the RIGHT) and multiply
        suffix: [24, 12, 4,  1]
                  ↑   ↑   ↑
                  │   │   └── 1 (nothing to the right of last index)
                  │   └────── 4
                  └────────── 4*3 = 12

        Combined (prefix * suffix):
        result: [24, 12, 8, 6]

Final result: [24, 12, 8, 6]
```

## Complexity Analysis Table

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Prefix-Suffix | O(n) | O(n) | No division |
| Division | O(n) | O(n) | Fails with zeros |

## Why Not Use Division?

If any element is zero, division by zero is undefined. Additionally, if there's a zero in the array, the product would be zero for ALL positions except the zero itself, which complicates the logic.

The prefix-suffix approach handles zeros naturally because:
- Position of zero will get left_product * right_product = non-zero * non-zero = non-zero (if zero is not at current position)

Actually wait - if there's a zero at position i, then answer[i] should be 0 (since we can't form a complete product), but answer[j != i] should also be 0 since zero is in the product.

The prefix-suffix approach correctly handles this case.

## Edge Cases to Consider

1. **Array with single element**: Return [1]
2. **Array with two elements**: Handle correctly
3. **Array with leading zeros**: Handle correctly
4. **Array with trailing zeros**: Handle correctly
5. **Array with middle zeros**: Return 0 for all positions
6. **Negative numbers**: Handle sign correctly
7. **Large products**: Use i64 to avoid overflow intermediate

## Related Problems

### LeetCode 238: Product of Array Except Self
Same problem.

### LeetCode 2376: Multiply Strings
Product without overflow.

### LeetCode 148: Sort List
Related through divide and conquer.

## Exercises

### Exercise 1: Basic Product Except Self
Implement using prefix-suffix approach.

### Exercise 2: Product Except Self with Division
Implement using division (for understanding edge cases).

### Exercise 3: Product Except Self with Zero Count
Handle zeros specifically.

### Exercise 4: Find All Products with Zeros
Return indices of all zeros in the array.

### Exercise 5: Product of Array Range
Return product of array elements in range [left, right].

## Key Takeaways

1. **Prefix and suffix** products are the key insight
2. **Two-pass algorithm** achieves O(n) time
3. **Division approach** fails with zeros
4. **Result[i]** = product of all elements except nums[i]
5. **Handle edge cases** for zeros and single elements

## Real-World Applications

1. **Signal processing**: Convolution operations
2. **Polynomial evaluation**: Without division
3. **Database queries**: Aggregate product operations
4. **Statistical analysis**: Products of observations
5. **Game scoring**: Combination of scores except current
