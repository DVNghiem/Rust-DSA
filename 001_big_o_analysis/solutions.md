# Solutions and Big O Analysis

This document provides detailed, line-by-line analysis of the time and space complexity for each exercise in the Big O Analysis module.

---

## Exercise 1: Sum of Array Elements

### sum_array function

```rust
pub fn sum_array(arr: &[i32]) -> i32 {
    let mut sum = 0;           // O(1) - single assignment
    for &val in arr {         // O(n) - iterates through n elements
        sum += val;            // O(1) per iteration - constant time operation
    }
    sum                       // O(1) - return
}
```

**Analysis:**
- **Time Complexity: O(n)** - The loop iterates through each of the n elements exactly once. Each iteration performs O(1) work (addition and assignment). Total: O(n) × O(1) = O(n).
- **Space Complexity: O(1)** - Only a single integer variable `sum` is allocated, regardless of input size. This is constant space.

### sum_array_recursive function

```rust
pub fn sum_array_recursive(arr: &[i32], n: usize) -> i32 {
    if n == 0 {               // O(1) - base case check
        return 0;             // O(1)
    }
    arr[n - 1] + sum_array_recursive(arr, n - 1)
    //  O(1) access + O(n-1) recursive call
}
```

**Analysis:**
- **Time Complexity: O(n)** - The function makes n recursive calls, each performing O(1) work (array access and addition). Total: O(n).
- **Space Complexity: O(n)** - Due to the call stack depth. Each recursive call adds a stack frame, and there are n frames at maximum depth. This is linear space.

---

## Exercise 2: Find Duplicates (Brute Force)

### has_duplicate function

```rust
pub fn has_duplicate(arr: &[i32]) -> bool {
    for i in 0..arr.len() {              // O(n) - outer loop
        for j in (i + 1)..arr.len() {    // O(n-i-1) on average
            if arr[i] == arr[j] {         // O(1) comparison
                return true;              // O(1) early exit
            }
        }
    }
    false                                // O(1)
}
```

**Analysis:**
- **Time Complexity: O(n²)** - The outer loop runs n times. For each i, the inner loop runs approximately (n-i-1) times. The total number of comparisons is: Σ(n-i-1) for i from 0 to n-1 = n(n-1)/2 = O(n²).
- **Space Complexity: O(1)** - Only a few variables are allocated. No data structures scale with input size.
- **Best Case: O(1)** - If the first two elements are duplicates, the function returns immediately.
- **Worst Case: O(n²)** - When there are no duplicates or duplicates are at the end.

---

## Exercise 3: Recursive Fibonacci

### fib (naive) function

```rust
pub fn fib(n: u64) -> u64 {
    if n <= 1 {           // O(1) comparison
        return n;         // O(1) return
    }
    fib(n - 1) + fib(n - 2)
    // T(n-1) + T(n-2) + O(1)
}
```

**Analysis:**
- **Time Complexity: O(2^n)** - This creates a binary tree of recursive calls. At each level, we make two calls (except for the base cases). The number of calls follows the Fibonacci sequence: F(n) ≈ 2^n / √n. This is exponential.
- **Space Complexity: O(n)** - Maximum call stack depth is n (when calculating fib(n), we go n levels deep before any call returns).
- **Note:** This is highly inefficient! Each subproblem is solved multiple times.

### fib_memoized function

```rust
pub fn fib_memoized(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if n <= 1 {               // O(1)
        return n;             // O(1)
    }
    if let Some(&result) = memo.get(&n) {  // O(1) HashMap lookup
        return result;
    }
    let result = fib_memoized(n - 1, memo) + fib_memoized(n - 2, memo);
    //                            O(n) calls with memoization
    memo.insert(n, result);    // O(1) HashMap insert
    result
}
```

**Analysis:**
- **Time Complexity: O(n)** - With memoization, each value from 0 to n is computed exactly once. Each computation takes O(1) time.
- **Space Complexity: O(n)** - The HashMap stores n entries, and the call stack depth is O(n).

---

## Exercise 4: Two Sum with HashMap

### two_sum function

```rust
pub fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut map = HashMap::new();           // O(1) - creates empty HashMap
    for (i, &num) in nums.iter().enumerate() {  // O(n) - single pass
        if let Some(&j) = map.get(&(target - num)) {  // O(1) average
            return Some((j, i));            // O(1)
        }
        map.insert(num, i);                  // O(1) average
    }
    None                                    // O(1)
}
```

**Analysis:**
- **Time Complexity: O(n)** - We make a single pass through the array. Each HashMap operation is O(1) average case.
- **Space Complexity: O(n)** - In the worst case, the HashMap stores n elements (when no solution exists).
- **Why HashMap?** - Allows O(1) lookup to check if (target - current) exists in previously seen elements.

---

## Exercise 5: Mystery Function Analysis

### mystery function

```rust
pub fn mystery(n: usize) -> usize {
    let mut count = 0;          // O(1)
    let mut i = n;
    while i > 0 {              // O(log n) iterations
        i /= 2;                // O(1) - halving
        let mut j = 0;
        while j < n {          // O(n) iterations per outer loop
            j += 1;            // O(1)
            count += 1;        // O(1)
        }
    }
    count                      // O(1)
}
```

**Analysis:**
- **Time Complexity: O(n log n)** - The outer loop runs log₂(n) times (halving i each iteration). For each outer iteration, the inner loop runs n times. Total: n × log(n) = O(n log n).
- **Space Complexity: O(1)** - Only a few scalar variables are used.

---

## Exercise 6: String Reversal

### reverse_string function

```rust
pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
    // O(n) to iterate, O(n) to collect into String
}
```

**Analysis:**
- **Time Complexity: O(n)** - `chars()` iterates through n characters, `rev()` reverses the iterator (no extra work), and `collect()` builds a new string of length n.
- **Space Complexity: O(n)** - A new String of length n is created.

### reverse_string_in_place function

```rust
pub fn reverse_string_in_place(s: &mut [char]) {
    let mut left = 0;                      // O(1)
    let mut right = s.len().saturating_sub(1);  // O(1)
    while left < right {                  // O(n/2) iterations
        s.swap(left, right);              // O(1)
        left += 1;                         // O(1)
        right = right.saturating_sub(1);   // O(1)
    }
}
```

**Analysis:**
- **Time Complexity: O(n)** - The loop swaps elements in pairs. With n elements, we perform n/2 swaps.
- **Space Complexity: O(1)** - No additional memory allocated; operates in-place.

---

## Exercise 7: BST Search

### bst_search function

```rust
pub fn bst_search(root: Option<&Node>, target: i32) -> bool {
    let mut current = root;                 // O(1)
    while let Some(node) = current {       // O(h) where h is height
        match target.cmp(&node.val) {     // O(1) comparison
            std::cmp::Ordering::Equal => return true,  // O(1)
            std::cmp::Ordering::Less => current = node.left.as_ref(),  // O(1)
            std::cmp::Ordering::Greater => current = node.right.as_ref(),  // O(1)
        }
    }
    false                                  // O(1)
}
```

**Analysis:**
- **Time Complexity: O(h)** where h is the height of the tree. In the worst case, we traverse from root to a leaf.
  - Best Case: O(1) - root is the target
  - Average Case (balanced tree): O(log n)
  - Worst Case (skewed tree): O(n)
- **Space Complexity: O(1)** - Only pointer manipulations, no extra memory allocation.

---

## Exercise 8: Recursive String Permutations

### permutations function

```rust
pub fn permutations(s: &str) -> Vec<String> {
    let mut result = Vec::new();           // O(1) initial capacity
    let chars: Vec<char> = s.chars().collect();  // O(n)
    fn helper(chars: &[char], start: usize, result: &mut Vec<String>) {
        if start == chars.len() {          // O(1)
            result.push(chars.iter().collect());  // O(n)
        } else {
            for i in start..chars.len() {  // O(n) recursive calls
                chars.swap(start, i);      // O(1)
                helper(chars, start + 1, result);
                chars.swap(start, i);      // O(1) backtrack
            }
        }
    }
    helper(&chars, 0, &mut result);        // initial call
    result
}
```

**Analysis:**
- **Time Complexity: O(n!)** - There are n! permutations of n characters. For each permutation, we do O(n) work to copy the characters.
- **Space Complexity: O(n!)** - We store n! strings, each of length n. Plus O(n) for the recursion depth.
- **Explanation:** The first character can be any of n positions, the second can be any of n-1 remaining positions, etc.

---

## Exercise 9: Sliding Window Maximum

### max_sliding_window function

```rust
pub fn max_sliding_window(nums: &[i32], k: usize) -> Vec<i32> {
    let mut result = Vec::new();            // O(1) amortized
    let mut deque = VecDeque::new();        // O(1)

    for (i, &num) in nums.iter().enumerate() {  // O(n)
        // Remove indices outside current window
        while let Some(&back) = deque.back() {
            if nums[back] < num {
                deque.pop_back();          // O(1)
            } else {
                break;
            }
        }
        deque.push_back(i);                 // O(1)
        // Remove indices outside window from front
        if deque.front() == Some(&(i - k)) {
            deque.pop_front();              // O(1)
        }
        // Record maximum when window is complete
        if i >= k.saturating_sub(1) {
            result.push(nums[*deque.front().unwrap()]);  // O(1)
        }
    }
    result
}
```

**Analysis:**
- **Time Complexity: O(n)** - Each element is pushed and popped from the deque at most once. All deque operations are O(1) amortized.
- **Space Complexity: O(k)** for the deque (worst case when all elements are in decreasing order), plus O(n) for the result.

---

## Exercise 10: Nested Loop with Early Exit

### find_pair_sum function

```rust
pub fn find_pair_sum(arr: &[i32], target: i32) -> Option<(i32, i32)> {
    for i in 0..arr.len() {                // O(n)
        for j in 0..arr.len() {             // O(n) for each i
            if arr[i] + arr[j] == target { // O(1)
                return Some((arr[i], arr[j]));  // O(1) early exit
            }
        }
    }
    None
}
```

**Analysis:**
- **Time Complexity:**
  - Best Case: O(1) - If the first pair sums to target
  - Average Case: O(n²/2) = O(n²) - Random position of target
  - Worst Case: O(n²) - No pair exists or target pair is at the end
- **Space Complexity: O(1)** - Only scalar variables.

---

## Exercise 11: All Pairs with Sum

### all_pairs_with_sum function

```rust
pub fn all_pairs_with_sum(nums: &[i32], target: i32) -> Vec<(usize, usize)> {
    let mut result = Vec::new();            // O(1) initial
    let mut map = HashMap::new();           // O(1)

    for (i, &num) in nums.iter().enumerate() {  // O(n)
        if let Some(&j) = map.get(&(target - num)) {  // O(1)
            result.push((j, i));           // O(1) amortized
        }
        map.insert(num, i);                 // O(1) amortized
    }
    result
}
```

**Analysis:**
- **Time Complexity: O(n)** - Single pass through the array.
- **Space Complexity: O(n)** - In worst case (no pairs), HashMap stores all n elements.

---

## Exercise 12: Recursive Binary Search

### binary_search_recursive function

```rust
pub fn binary_search_recursive(arr: &[i32], target: i32, left: usize, right: usize) -> Option<usize> {
    if left >= right {                     // O(1)
        return None;                       // O(1)
    }
    let mid = left + (right - left) / 2;   // O(1)
    match arr[mid].cmp(&target) {          // O(1)
        std::cmp::Ordering::Equal => Some(mid),  // O(1)
        std::cmp::Ordering::Less => binary_search_recursive(arr, target, mid + 1, right),
        std::cmp::Ordering::Greater => binary_search_recursive(arr, target, left, mid),
    }
}
```

**Analysis:**
- **Time Complexity: O(log n)** - Each recursive call halves the search space.
- **Space Complexity: O(log n)** - Call stack depth is O(log n).

---

## Exercise 13: Insertion Sort

### insertion_sort function

```rust
pub fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {               // O(n)
        let key = arr[i];                 // O(1)
        let mut j = i;                    // O(1)
        while j > 0 && arr[j - 1] > key { // O(n) in worst case
            arr[j] = arr[j - 1];          // O(1)
            j -= 1;                       // O(1)
        }
        arr[j] = key;                      // O(1)
    }
}
```

**Analysis:**
- **Time Complexity:**
  - Best Case: O(n) - Already sorted, inner loop never runs
  - Average Case: O(n²) - Elements are randomly distributed
  - Worst Case: O(n²) - Sorted in reverse order
- **Space Complexity: O(1)** - In-place sorting.

---

## Exercise 14: Power Function (Binary Exponentiation)

### power function

```rust
pub fn power(base: f64, exponent: i32) -> f64 {
    if exponent < 0 {
        return 1.0 / power(base, -exponent);  // Handle negatives
    }
    if exponent == 0 { return 1.0; }          // Base case
    if exponent == 1 { return base; }         // Base case

    let half = power(base, exponent / 2);      // T(n/2)
    if exponent % 2 == 0 {
        half * half                           // Even: x^2n = (x^n)^2
    } else {
        half * half * base                     // Odd: x^2n+1 = (x^n)^2 * x
    }
}
```

**Analysis:**
- **Time Complexity: O(log n)** - Each recursive call halves the exponent.
- **Space Complexity: O(log n)** - Call stack depth is O(log n).

---

## Exercise 15: Merge Two Sorted Arrays

### merge_sorted function

```rust
pub fn merge_sorted(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(arr1.len() + arr2.len());  // O(1)
    let mut i = 0;                          // O(1)
    let mut j = 0;                           // O(1)

    while i < arr1.len() && j < arr2.len() { // O(n + m)
        if arr1[i] <= arr2[j] {
            result.push(arr1[i]);            // O(1) amortized
            i += 1;
        } else {
            result.push(arr2[j]);
            j += 1;
        }
    }

    while i < arr1.len() {                   // O(n)
        result.push(arr1[i]);
        i += 1;
    }

    while j < arr2.len() {                    // O(m)
        result.push(arr2[j]);
        j += 1;
    }

    result                                   // O(1) return
}
```

**Analysis:**
- **Time Complexity: O(n + m)** where n and m are the lengths of the two arrays. Each element is visited exactly once.
- **Space Complexity: O(n + m)** - The result vector stores all elements.

---

## Summary Table

| Exercise | Function | Time | Space | Notes |
|----------|----------|------|-------|-------|
| 1a | sum_array | O(n) | O(1) | Linear pass |
| 1b | sum_array_recursive | O(n) | O(n) | Stack depth n |
| 2 | has_duplicate | O(n²) | O(1) | Nested loops |
| 3a | fib (naive) | O(2^n) | O(n) | Exponential |
| 3b | fib_memoized | O(n) | O(n) | Memoization |
| 4 | two_sum | O(n) | O(n) | HashMap |
| 5 | mystery | O(n log n) | O(1) | Halving outer |
| 6a | reverse_string | O(n) | O(n) | New string |
| 6b | reverse_string_in_place | O(n) | O(1) | In-place |
| 7 | bst_search | O(h) | O(1) | h = height |
| 8 | permutations | O(n!) | O(n!) | Factorial |
| 9 | max_sliding_window | O(n) | O(k) | Deque |
| 10 | find_pair_sum | O(n²) | O(1) | Worst case |
| 11 | all_pairs_with_sum | O(n) | O(n) | HashMap |
| 12 | binary_search_recursive | O(log n) | O(log n) | Recursive |
| 13 | insertion_sort | O(n²) | O(1) | Worst case |
| 14 | power | O(log n) | O(log n) | Binary exp |
| 15 | merge_sorted | O(n+m) | O(n+m) | Linear merge |

This analysis should give you a solid foundation for analyzing any algorithm's complexity in your interviews and daily work.
