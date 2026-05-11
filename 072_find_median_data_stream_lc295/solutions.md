# Find Median from Data Stream Solution - LeetCode 295 (Complete)

## Solution Analysis

### Two Heaps Approach

```rust
pub struct MedianFinder {
    lo: BinaryHeap<i32>,                  // max-heap (lower half)
    hi: BinaryHeap<Reverse<i32>>,       // min-heap (upper half)
}

impl MedianFinder {
    pub fn new() -> Self {
        MedianFinder {
            lo: BinaryHeap::new(),
            hi: BinaryHeap::new(),
        }
    }

    pub fn add_num(&mut self, num: i32) {
        // Add to max-heap (lower half)
        self.lo.push(num);

        // Move largest of lower half to min-heap (upper half)
        if let Some(max_lo) = self.lo.pop() {
            self.hi.push(Reverse(max_lo));
        }

        // Balance: ensure lo has >= elements than hi
        if self.hi.len() > self.lo.len() {
            if let Some(min_hi) = self.hi.pop() {
                self.lo.push(min_hi.0);
            }
        }
    }

    pub fn find_median(&self) -> f64 {
        if self.lo.is_empty() {
            return 0.0;
        }

        if self.lo.len() > self.hi.len() {
            *self.lo.peek().unwrap() as f64
        } else {
            (*self.lo.peek().unwrap() as f64 + self.hi.peek().unwrap().0 as f64) / 2.0
        }
    }
}
```

## Why Two Heaps?

### Key Insight

- **lo (max-heap)**: Lower half of numbers, largest at top
- **hi (min-heap)**: Upper half of numbers, smallest at top
- Median is either top of lo, or average of tops of lo and hi

### Balance Invariant

```
|lo| = |hi|     OR     |lo| = |hi| + 1
```

When odd count: lo has one extra element (the median)
When even count: take average of tops

## Line-by-Line Analysis

### Add Number

```rust
// 1. Add to lower half (lo)
self.lo.push(num);

// 2. Move max of lo to hi (balance)
if let Some(max_lo) = self.lo.pop() {
    self.hi.push(Reverse(max_lo));
}

// 3. If hi has more, rebalance
if self.hi.len() > self.lo.len() {
    if let Some(min_hi) = self.hi.pop() {
        self.lo.push(min_hi.0);
    }
}
```

**Intuition**:
- Always push to lo first (we want numbers <= median in lo)
- Move top of lo to hi (numbers > median go to hi)
- If hi gets too big, move one back to lo

### Find Median

```rust
if self.lo.len() > self.hi.len() {
    *self.lo.peek().unwrap() as f64  // Odd: middle is in lo
} else {
    (*self.lo.peek().unwrap() as f64 + self.hi.peek().unwrap().0 as f64) / 2.0
    // Even: average of two middles
}
```

## Visual Example

### Add Sequence: 1, 2, 3, 4, 5

```
Add 1:
  lo = [1], hi = []
  median = 1

Add 2:
  lo = [1], push 2 → lo = [2, 1]
  move max to hi: max(2) → hi = [2], lo = [1]
  hi has more? No
  median = 1

Add 3:
  lo = [1], push 3 → lo = [3, 1]
  move max to hi: max(3) → hi = [2, 3], lo = [1]
  hi has more? Yes (2 > 1), move min from hi to lo
  hi = [3], lo = [1, 2]
  lo size > hi, median = top of lo = 2

Add 4:
  lo = [1, 2], push 4 → lo = [4, 1, 2]
  move max to hi: max(4) → hi = [3, 4], lo = [1, 2]
  hi has more? No (2 = 2)
  median = (top of lo + top of hi) / 2 = (2 + 3) / 2 = 2.5

Add 5:
  lo = [1, 2], push 5 → lo = [5, 1, 2]
  move max to hi: max(5) → hi = [3, 4, 5], lo = [1, 2]
  hi has more? Yes (3 > 2), move min from hi to lo
  hi = [4, 5], lo = [1, 2, 3]
  lo size > hi, median = top of lo = 3
```

## Why This Works

### Invariant Maintained

After each add:
- All elements in lo are <= all elements in hi
- |lo| = |hi| or |lo| = |hi| + 1

This ensures median is always accessible:
- If |lo| > |hi|: median is top of lo
- If |lo| = |hi|: median is average of tops

## Complexity Analysis

| Operation | Time |
|-----------|------|
| add_num | O(log n) |
| find_median | O(1) |

Both heaps maintain balance, each operation is O(log n).

## Edge Cases

### Empty
```rust
MedianFinder::new()
find_median() → 0.0
```

### Single Element
```rust
add_num(5)
find_median() → 5.0
```

### Two Elements
```rust
add_num(1)
add_num(3)
find_median() → 2.0
```

## Common Mistakes

1. **Confusing heap types**: lo is max-heap, hi is min-heap (via Reverse)
2. **Not balancing**: Could lead to incorrect median
3. **Integer division**: Must convert to f64 before averaging