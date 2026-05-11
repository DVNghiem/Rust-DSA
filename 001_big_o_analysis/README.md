# Big O Analysis

## Introduction

Big O notation is a mathematical notation that describes the limiting behavior of a function when the argument tends towards a particular value or infinity. In computer science, Big O notation is used to classify algorithms according to how their run time or space requirements grow as the input size grows.

Understanding Big O is crucial for every software engineer because it allows us to:
1. Predict how an algorithm will perform as input size increases
2. Compare the efficiency of different algorithms for the same problem
3. Identify performance bottlenecks before they become issues
4. Make informed decisions about data structures and algorithm choices

## Time Complexity Classes

### O(1) - Constant Time

An algorithm is O(1) if the execution time does not depend on the input size. Regardless of how large the input is, the operation takes the same amount of time.

```rust
// O(1) - Accessing an element by index
fn get_element(arr: &[i32], index: usize) -> Option<i32> {
    if index < arr.len() {
        Some(arr[index])
    } else {
        None
    }
}
```

### O(log n) - Logarithmic Time

An algorithm is O(log n) if the time grows logarithmically as input size increases. This typically happens with algorithms that divide the problem in half each step.

```rust
// O(log n) - Binary search
fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;
        match arr[mid].cmp(&target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid,
        }
    }
    None
}
```

Visual representation of binary search:
```
Array: [1, 3, 5, 7, 9, 11, 13, 15]
Search for: 7

Step 1: [1, 3, 5, 7, 9, 11, 13, 15]
            ↑
            mid=7 (arr[3]=7) ✓ Found!

Each step halves the search space:
n=8 → 4 → 2 → 1 = log₂(8) = 3 steps
```

### O(n) - Linear Time

An algorithm is O(n) if the time grows linearly with input size.

```rust
// O(n) - Linear search
fn linear_search(arr: &[i32], target: i32) -> Option<usize> {
    for (i, &val) in arr.iter().enumerate() {
        if val == target {
            return Some(i);
        }
    }
    None
}
```

### O(n log n) - Linearithmic Time

This complexity appears in efficient sorting algorithms like Merge Sort, Heap Sort, and Quick Sort (average case).

```rust
// O(n log n) - Merge sort divide step
fn merge_sort_helper(arr: &[i32], left: usize, right: usize) -> Vec<i32> {
    if left >= right {
        return vec![arr[left]];
    }

    let mid = left + (right - left) / 2;
    let left_sorted = merge_sort_helper(arr, left, mid);
    let right_sorted = merge_sort_helper(arr, mid + 1, right);

    merge(&left_sorted, &right_sorted)
}
```

Visual of merge sort:
```
[8, 3, 7, 1, 5, 6, 2, 4]
       ↓ divide
[8, 3, 7, 1] [5, 6, 2, 4]
       ↓ divide
[8, 3] [7, 1] [5, 6] [2, 4]
       ↓ divide
[8] [3] [7] [1] [5] [6] [2] [4]
       ↓ merge
[3, 8] [1, 7] [5, 6] [2, 4]
       ↓ merge
[1, 3, 7, 8] [2, 4, 5, 6]
       ↓ merge
[1, 2, 3, 4, 5, 6, 7, 8]

Each level does O(n) work, there are log(n) levels = O(n log n)
```

### O(n²) - Quadratic Time

Algorithms that compare all pairs of elements typically have O(n²) complexity.

```rust
// O(n²) - Bubble sort
fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}
```

Visual of bubble sort:
```
[5, 3, 8, 1, 2]
Pass 1: [3, 5, 1, 2, 8] - largest bubbles up
Pass 2: [3, 1, 2, 5, 8]
Pass 3: [1, 2, 3, 5, 8]
Pass 4: [1, 2, 3, 5, 8]

n elements × n passes = O(n²)
```

### O(2^n) - Exponential Time

Algorithms that double in time with each additional input element.

```rust
// O(2^n) - Generate all subsets
fn generate_subsets(nums: &[i32]) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let n = nums.len();

    for mask in 0..(1 << n) {
        let mut subset = Vec::new();
        for i in 0..n {
            if mask & (1 << i) != 0 {
                subset.push(nums[i]);
            }
        }
        result.push(subset);
    }
    result
}
```

### O(n!) - Factorial Time

The worst commonly encountered complexity. Example: generating all permutations.

```rust
// O(n!) - Generate all permutations (inefficient)
fn permutations(vec: &mut Vec<i32>, left: usize, right: usize, result: &mut Vec<Vec<i32>>) {
    if left == right {
        result.push(vec.clone());
    } else {
        for i in left..=right {
            vec.swap(left, i);
            permutations(vec, left + 1, right, result);
            vec.swap(left, i);
        }
    }
}
```

## Space Complexity

Space complexity measures the amount of memory an algorithm uses relative to input size.

### In-place vs Out-of-place

An in-place algorithm uses O(1) extra space (beyond the input). An out-of-place algorithm uses additional space that grows with input.

```rust
// O(1) space - In-place swap
fn swap_in_place(arr: &mut [i32], i: usize, j: usize) {
    arr[i] ^= arr[j];
    arr[j] ^= arr[i];
    arr[i] ^= arr[j];
}

// O(n) space - Create new array
fn reverse_copy(arr: &[i32]) -> Vec<i32> {
    arr.iter().rev().copied().collect()
}
```

### Memory Growth Examples

```rust
// O(n) space - Store all prefix sums
fn prefix_sums(arr: &[i32]) -> Vec<i32> {
    let mut prefix = Vec::with_capacity(arr.len());
    let mut sum = 0;
    for &val in arr {
        sum += val;
        prefix.push(sum);
    }
    prefix
}
```

## Best, Average, and Worst Case Analysis

Different inputs can cause the same algorithm to have different complexities.

### Example: Quick Sort

```
Best Case: O(n log n) - Pivot always lands in middle
Average Case: O(n log n) - Random data
Worst Case: O(n²) - Already sorted data with bad pivot choice
```

### Example: Linear Search

```
Best Case: O(1) - Target is first element
Average Case: O(n/2) = O(n) - Target is randomly positioned
Worst Case: O(n) - Target is last or not present
```

## Comprehensive Big O Table

| Complexity | Name | 10 items | 100 items | 1000 items |
|------------|------|----------|-----------|------------|
| O(1) | Constant | 1 | 1 | 1 |
| O(log n) | Logarithmic | 3 | 7 | 10 |
| O(n) | Linear | 10 | 100 | 1000 |
| O(n log n) | Linearithmic | 30 | 700 | 10000 |
| O(n²) | Quadratic | 100 | 10000 | 1000000 |
| O(2^n) | Exponential | 1024 | 2^100 | 2^1000 |
| O(n!) | Factorial | 3628800 | huge | astronomical |

## Master Theorem

The Master Theorem provides a way to analyze recursive algorithms:

```
If T(n) = aT(n/b) + f(n):
- If f(n) = O(n^log_b(a-ε)) for ε > 0: T(n) = O(n^log_b(a))
- If f(n) = O(n^log_b(a)): T(n) = O(n^log_b(a) log n)
- If f(n) = O(n^log_b(a+ε)) for ε > 0: T(n) = O(f(n))
```

### Example: Merge Sort

```
T(n) = 2T(n/2) + O(n)
a = 2, b = 2, f(n) = O(n)
n^log_b(a) = n^log_2(2) = n^1 = n
Since f(n) = O(n^log_b(a)), case 2 applies: T(n) = O(n log n)
```

## Common Data Structure Operations

| Data Structure | Access | Search | Insert | Delete |
|----------------|--------|--------|--------|--------|
| Array | O(1) | O(n) | O(n) | O(n) |
| HashMap | N/A | O(1)* | O(1)* | O(1)* |
| BST (balanced) | O(log n) | O(log n) | O(log n) | O(log n) |
| Linked List | O(n) | O(n) | O(1) | O(1)** |
| Stack | N/A | O(n) | O(1) | O(1) |
| Queue | N/A | O(n) | O(1) | O(1) |

* Average case, assumes good hash function
** When node reference is available

## Four Approaches for Problem Solving

When analyzing an algorithm, consider these four approaches:

### 1. Brute Force
Try all possibilities until you find a solution.
- Often O(n!) or O(2^n)
- Good for small inputs or when correctness matters more than speed

### 2. Divide and Conquer
Split the problem, solve subproblems, combine results.
- Typical complexity: O(n log n)
- Examples: Merge Sort, Binary Search, Quick Sort

### 3. Dynamic Programming
Store subproblem results to avoid recomputation.
- Typical complexity: O(n) or O(n²)
- Examples: Fibonacci, Knapsack, Longest Common Subsequence

### 4. Greedy
Make locally optimal choices at each step.
- Doesn't always guarantee global optimum
- Examples: Huffman coding, Dijkstra's algorithm

## Exercises

Analyze the time and space complexity of the following algorithms:

### Exercise 1: Sum of Array Elements
```rust
fn sum_array(arr: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in arr {
        sum += val;
    }
    sum
}
```

### Exercise 2: Find Duplicates
```rust
fn has_duplicate(arr: &[i32]) -> bool {
    for i in 0..arr.len() {
        for j in (i + 1)..arr.len() {
            if arr[i] == arr[j] {
                return true;
            }
        }
    }
    false
}
```

### Exercise 3: Recursive Fibonacci
```rust
fn fib(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}
```

### Exercise 4: Two Sum with HashMap
```rust
fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        if let Some(&j) = map.get(&(target - num)) {
            return Some((j, i));
        }
        map.insert(num, i);
    }
    None
}
```

### Exercise 5: Nested Loop with Division
```rust
fn mystery(n: usize) -> usize {
    let mut count = 0;
    let mut i = n;
    while i > 0 {
        i /= 2;
        let mut j = 0;
        while j < n {
            j += 1;
            count += 1;
        }
    }
    count
}
```

### Exercise 6: String Reversal
```rust
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}
```

### Exercise 7: Binary Search Tree Search
```rust
fn bst_search(root: Option<&Node>, target: i32) -> bool {
    let mut current = root;
    while let Some(node) = current {
        match target.cmp(&node.val) {
            std::cmp::Ordering::Equal => return true,
            std::cmp::Ordering::Less => current = node.left.as_ref(),
            std::cmp::Ordering::Greater => current = node.right.as_ref(),
        }
    }
    false
}
```

### Exercise 8: Recursive String Permutations
```rust
fn permutations(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    let chars: Vec<char> = s.chars().collect();

    fn helper(chars: &[char], start: usize, result: &mut Vec<String>) {
        if start == chars.len() {
            result.push(chars.iter().collect());
        } else {
            for i in start..chars.len() {
                chars.swap(start, i);
                helper(chars, start + 1, result);
                chars.swap(start, i);
            }
        }
    }

    helper(&chars, 0, &mut result);
    result
}
```

### Exercise 9: Sliding Window Maximum
```rust
fn max_sliding_window(nums: &[i32], k: usize) -> Vec<i32> {
    use std::collections::VecDeque;
    let mut result = Vec::new();
    let mut deque = VecDeque::new();

    for (i, &num) in nums.iter().enumerate() {
        while let Some(&back) = deque.back() {
            if nums[back] < num {
                deque.pop_back();
            } else {
                break;
            }
        }
        deque.push_back(i);

        if deque.front() == Some(&(i - k)) {
            deque.pop_front();
        }

        if i >= k - 1 {
            result.push(nums[*deque.front().unwrap()]);
        }
    }
    result
}
```

### Exercise 10: Nested Loop with Early Exit
```rust
fn find_pair_sum(arr: &[i32], target: i32) -> Option<(i32, i32)> {
    for i in 0..arr.len() {
        for j in 0..arr.len() {
            if arr[i] + arr[j] == target {
                return Some((arr[i], arr[j]));
            }
        }
    }
    None
}
```

## Summary

Understanding Big O notation is fundamental to writing efficient code. Key takeaways:

1. **Focus on the dominant term**: Drop constants and lower-order terms
2. **Consider all three cases**: Best, average, and worst case
3. **Remember space complexity**: Memory usage matters too
4. **Use the Master Theorem** for recursive algorithms
5. **Practice analyzing** different algorithm patterns

In the following sections, we will apply these Big O concepts to solve specific LeetCode problems, starting with Two Sum and progressing through arrays, linked lists, stacks, queues, and binary trees.
