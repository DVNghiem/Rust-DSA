# Solutions: Product of Array Except Self (LeetCode #238)

This document provides detailed, line-by-line analysis of each exercise implementation.

---

## Exercise 1: Product Except Self - Prefix Suffix

### The Solution

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

### Line-by-Line Analysis

```rust
let n = nums.len();
let mut result = vec![1; n];
```
**Purpose:** Initialize result array with 1s. `result[i]` will store the product of all elements to the left of `i`.

```rust
let mut prefix = 1;
for i in 0..n {
    result[i] = prefix;
    prefix *= nums[i];
}
```
**Purpose:** First pass - calculate prefix products.
- `prefix` starts at 1 (product of no elements)
- `result[i]` gets the product of all elements to the LEFT of i
- `prefix` then includes `nums[i]` for the next iteration

```rust
let mut suffix = 1;
for i in (0..n).rev() {
    result[i] *= suffix;
    suffix *= nums[i];
}
```
**Purpose:** Second pass (right to left) - calculate suffix products and combine.
- `suffix` starts at 1 (product of no elements to the right)
- `result[i]` is multiplied by product of all elements to the RIGHT of i
- `suffix` then includes `nums[i]` for the next iteration

### Complexity Analysis

| Metric | Value | Explanation |
|--------|-------|-------------|
| **Time** | O(n) | Two linear passes |
| **Space** | O(n) | Result array |

### Why This Works

For position i:
- `result[i]` starts as product of all elements to the left of i (from first pass)
- After second pass, `result[i]` is multiplied by product of all elements to the right of i

The result is: (product of left elements) × (product of right elements) = product of all elements except nums[i]

---

## Exercise 2: Product Except Self with Division

### The Solution

```rust
pub fn product_except_self_with_divide(nums: &[i32]) -> Vec<i32> {
    let total: i64 = nums.iter().map(|&x| x as i64).product();
    let zero_count = nums.iter().filter(|&&x| x == 0).count();

    match zero_count {
        0 => nums.iter().map(|&x| (total / x as i64) as i32).collect(),
        1 => nums.iter().map(|&x| if x == 0 { total as i32 } else { 0 }).collect(),
        _ => vec![0; nums.len()],
    }
}
```

### Line-by-Line Analysis

```rust
let total: i64 = nums.iter().map(|&x| x as i64).product();
```
**Purpose:** Calculate total product using i64 to avoid overflow. `product()` is a standard iterator method.

```rust
let zero_count = nums.iter().filter(|&&x| x == 0).count();
```
**Purpose:** Count how many zeros are in the array. This determines our strategy.

```rust
match zero_count {
    0 => nums.iter().map(|&x| (total / x as i64) as i32).collect(),
```
**Purpose:** No zeros - simple division works. For each element, total / element gives the desired product.

```rust
1 => nums.iter().map(|&x| if x == 0 { total as i32 } else { 0 }).collect(),
```
**Purpose:** Exactly one zero:
- If x is 0: answer should be total (product of all non-zero elements)
- If x is non-zero: answer should be 0 (the zero makes product zero)

```rust
_ => vec![0; nums.len()],
```
**Purpose:** Multiple zeros - every position includes at least one zero, so all products are 0.

### Why O(n) but Not Preferred

1. Division can cause overflow issues with very large numbers
2. Division by zero must be handled explicitly
3. The problem explicitly asks to avoid division
4. Prefix-suffix works correctly for all cases without special handling

---

## Exercise 3: Explicit Zero Handling

### The Solution

```rust
pub fn product_except_self_with_zeros(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();
    let zero_positions: Vec<usize> = nums.iter().enumerate()
        .filter(|(_, &&x)| x == 0)
        .map(|(i, _)| i)
        .collect();

    if zero_positions.len() > 1 {
        return vec![0; n];
    }

    if let Some(zpos) = zero_positions.first() {
        // One zero: that position gets product of all non-zeros
        let mut product = 1i64;
        for &x in nums.iter().filter(|&&x| x != 0) {
            product *= x as i64;
        }
        let mut result = vec![0; n];
        result[zpos] = product as i32;
        return result;
    }

    // No zeros: use prefix-suffix
    let mut result = vec![1; n];
    let mut prefix = 1i64;
    for i in 0..n {
        result[i] = prefix as i32;
        prefix *= nums[i] as i64;
    }
    let mut suffix = 1i64;
    for i in (0..n).rev() {
        result[i] *= suffix as i32;
        suffix *= nums[i] as i64;
    }
    result
}
```

### Logic Flow

1. Find all zero positions
2. If more than 1 zero: all results are 0
3. If exactly 1 zero: that position gets product of non-zeros, others get 0
4. If no zeros: use prefix-suffix approach

---

## Exercise 5: Product Range Query

### The Solution

```rust
pub struct ProductQuery {
    prefix_products: Vec<i64>,
}

impl ProductQuery {
    pub fn new(nums: &[i32]) -> Self {
        let mut prefix_products = vec![1i64; nums.len() + 1];
        for i in 0..nums.len() {
            prefix_products[i + 1] = prefix_products[i] * (nums[i] as i64);
        }
        ProductQuery { prefix_products }
    }

    pub fn product(&self, left: usize, right: usize) -> i64 {
        self.prefix_products[right + 1] / self.prefix_products[left]
    }
}
```

### Key Insight

Using prefix products where `prefix[i]` = product of `nums[0..i]`, we can compute any range product as:
`prefix[right + 1] / prefix[left]`

This is a classic prefix sum pattern, adapted for products.

---

## Summary Table

| Exercise | Time | Space | Key Technique |
|----------|------|-------|---------------|
| 1: Prefix-Suffix | O(n) | O(n) | Two-pass |
| 2: Division | O(n) | O(n) | Case analysis |
| 3: Zero Handling | O(n) | O(n) | Zero detection |
| 4: Find Zeros | O(n) | O(k) | Enumerate |
| 5: Range Query | O(n)/O(1) | O(n) | Prefix products |
| 6: i64 Version | O(n) | O(n) | Larger type |
| 7: Verify | O(n) | O(1) | Direct check |
| 8: Subarray K | O(n) | O(1) | Sliding window |

## Key Takeaways

1. **Prefix-suffix** is the optimal O(n) solution
2. **Division fails** with zeros and causes overflow
3. **Handle zeros** by detecting and special-casing
4. **i64 prevents overflow** in intermediate calculations
5. **Range query** uses prefix product pattern
