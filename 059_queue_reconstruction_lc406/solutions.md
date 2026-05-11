# Queue Reconstruction by Height Solution - LeetCode 406 (Complete)

## Solution Analysis

### Sort by Height (Descending), Then Insert by k

```rust
pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut people = people;
    // Sort: height descending, then k ascending
    people.sort_by(|a, b| {
        if a[0] != b[0] {
            b[0].cmp(&a[0]) // taller first
        } else {
            a[1].cmp(&b[1]) // smaller k first
        }
    });

    let mut result: Vec<Vec<i32>> = Vec::new();
    for p in &people {
        let k = p[1] as usize;
        result.insert(k, p.clone());
    }

    result
}
```

## Key Insight

### Why Sort by Height Descending?

When we process people from tallest to shortest:
- All previously placed people are taller or equal height
- For the current person, we only need to count taller people
- `k` directly tells us the position among taller people

### Why k = Number of People in Front?

`people[i] = [h, k]` means:
- There are exactly `k` people with height >= `h` ahead of this person

When we insert by `k`, we place the person at position where there are `k` people already (all of which must be taller since we process tallest first).

## Line-by-Line Analysis

### Sorting Logic
```rust
people.sort_by(|a, b| {
    if a[0] != b[0] {
        b[0].cmp(&a[0]) // Height descending (taller first)
    } else {
        a[1].cmp(&b[1]) // k ascending (smaller first)
    }
});
```
- Primary sort: height descending (taller people first)
- Secondary sort: for same height, smaller k first

### Insert by k
```rust
let mut result: Vec<Vec<i32>> = Vec::new();
for p in &people {
    let k = p[1] as usize;
    result.insert(k, p.clone());
}
```
- Insert each person at position `k`
- This works because all previously inserted people are taller

## Visual Step-by-Step

### Input: people = [[7,0],[4,4],[7,1],[5,0],[6,1],[5,2]]

After sorting by height desc:
```
[[7,0], [7,1], [6,1], [5,0], [5,2], [4,4]]
```

Insert step by step:
```
Insert [7,0] at 0: [[7,0]]
Insert [7,1] at 1: [[7,0], [7,1]]
Insert [6,1] at 1: [[7,0], [6,1], [7,1]]
Insert [5,0] at 0: [[5,0], [7,0], [6,1], [7,1]]
Insert [5,2] at 2: [[5,0], [7,0], [5,2], [6,1], [7,1]]
Insert [4,4] at 4: [[5,0], [7,0], [5,2], [6,1], [4,4], [7,1]]
```

## Why This Works

### Invariant

At each step, `result` contains correctly positioned people among all already processed (taller) people.

### Proof Sketch

1. Base: Empty `result` is trivially correct
2. Inductive: When inserting person `p` with height `h` and `k`:
   - All people in `result` are taller or equal to `h`
   - We insert at position `k`
   - There are exactly `k` people before insertion point (all taller)
   - After insertion, `p` has exactly `k` taller people in front ✓

## Complexity Analysis

| Approach | Time | Space |
|----------|------|-------|
| Sort + Insert | O(n²) | O(n) |
| Deferred Insert (LinkedList) | O(n log n) | O(n) |

Insert operation is O(n), done n times = O(n²).

## Edge Cases

### Empty Input
```rust
Input: []
Output: []
```

### Single Person
```rust
Input: [[5, 0]]
Output: [[5, 0]]
```

### Multiple Same Height
```rust
Input: [[5, 0], [5, 1], [5, 2]]
After sort: [[5, 0], [5, 1], [5, 2]]
Insert 0: [[5, 0]]
Insert 1: [[5, 0], [5, 1]]
Insert 2: [[5, 0], [5, 1], [5, 2]]
```

## Common Mistakes

1. **Sorting ascending by height**: Would break the algorithm
2. **Not handling k correctly**: k could be any valid index, not necessarily where we think
3. **Confusing h and k**: h is height, k is number of taller/equal people in front

## Why Insert at k Works

Consider person [7, 1]:
- Means: there is 1 person with height >= 7 in front
- When we insert [7, 1], there is already 1 person (taller) in result
- Inserting at position 1 puts this person after exactly 1 person
- All people before are taller (because we process tallest first)