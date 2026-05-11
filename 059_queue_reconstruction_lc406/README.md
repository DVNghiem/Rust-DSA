# Queue Reconstruction by Height - LeetCode 406

## Problem Overview

Given an array of people `people` where `people[i] = [h, k]` represents height `h` and people with the same height must have `k` people in front, return the reconstructed queue.

**Examples:**
```
Input: people = [[7,0],[4,4],[7,1],[5,0],[6,1],[5,2]]
Output: [[5,0],[6,1],[7,0],[7,1],[4,4],[5,2]]
```

## Theory

### Key Insight: Sort by Height Descending, Then Insert by k

When we process people from tallest to shortest, all previously placed people are taller or equal height. So we can insert by k position directly.

```
Sort by height desc: [[7,0],[7,1],[6,1],[5,0],[5,2],[4,4]]

Insert by k:
  [7,0] → insert at 0     → [[7,0]]
  [7,1] → insert at 1     → [[7,0],[7,1]]
  [6,1] → insert at 1     → [[7,0],[6,1],[7,1]]
  [5,0] → insert at 0     → [[5,0],[7,0],[6,1],[7,1]]
  [5,2] → insert at 2     → [[5,0],[7,0],[5,2],[6,1],[7,1]]
  [4,4] → insert at 4     → [[5,0],[7,0],[5,2],[6,1],[4,4],[7,1]]
```

## Implementation

```rust
pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut people = people;
    people.sort_by(|a, b| {
        if a[0] != b[0] { b[0].cmp(&a[0]) } else { a[1].cmp(&b[1]) }
    });

    let mut result: Vec<Vec<i32>> = Vec::new();
    for p in &people {
        let k = p[1] as usize;
        result.insert(k, p.clone());
    }

    result
}
```

## Complexity Analysis

| Approach | Time | Space |
|----------|------|-------|
| Sort + Insert | O(n²) | O(n) |

## Test Cases

```rust
#[test]
fn test_reconstruct_basic() {
    let result = reconstruct_queue(vec![vec![7,0],vec![4,4],vec![7,1],vec![5,0],vec![6,1],vec![5,2]]);
    assert_eq!(result.len(), 6);
}
```