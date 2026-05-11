# Kth Largest Element in a Stream Solution - LeetCode 703 (Complete)

## Solution Analysis

### Min-Heap of Size k

```rust
pub struct KthLargest {
    k: usize,
    heap: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    pub fn new(k: i32, mut nums: Vec<i32>) -> Self {
        let k = k as usize;
        let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

        for num in nums.drain(..) {
            if heap.len() < k {
                heap.push(Reverse(num));
            } else if num > heap.peek().unwrap().0 {
                heap.push(Reverse(num));
                heap.pop();
            }
        }

        KthLargest { k, heap }
    }

    pub fn add(&mut self, val: i32) -> i32 {
        if self.heap.len() < self.k {
            self.heap.push(Reverse(val));
        } else if val > self.heap.peek().unwrap().0 {
            self.heap.push(Reverse(val));
            self.heap.pop();
        }
        self.heap.peek().map(|r| r.0).unwrap_or(0)
    }
}
```

## Why Min-Heap?

### Key Insight

For kth largest:
- We need to find the kth largest element
- If we keep only the k largest elements in a min-heap
- The ROOT of min-heap = smallest of the k largest = kth largest

### Example

Elements seen: [4, 5, 8, 2], k = 3

After processing all:
- Heap contains [2, 4, 5] (the 3 largest elements)
- Heap root = 2 = 3rd largest ✓

## Line-by-Line Analysis

### Initialization
```rust
for num in nums.drain(..) {
    if heap.len() < k {
        heap.push(Reverse(num));
    } else if num > heap.peek().unwrap().0 {
        heap.push(Reverse(num));
        heap.pop();
    }
}
```
- While heap not full, push elements
- When full, only push if element is larger than smallest (root)
- This maintains only k largest elements

### Why Reverse?

```rust
BinaryHeap<Reverse<i32>>
```
- Rust's BinaryHeap is a max-heap by default
- Using `Reverse<i32>` makes it a min-heap
- Root = smallest of the k largest = kth largest

### Add Method
```rust
pub fn add(&mut self, val: i32) -> i32 {
    if self.heap.len() < self.k {
        self.heap.push(Reverse(val));
    } else if val > self.heap.peek().unwrap().0 {
        self.heap.push(Reverse(val));
        self.heap.pop();
    }
    self.heap.peek().map(|r| r.0).unwrap_or(0)
}
```
- If heap not full, just add
- If heap full and val > root, replace root
- Return root (kth largest)

## Visual Example

### Initial: k = 3, nums = [4, 5, 8, 2]

```
Add 4: heap = [4]
Add 5: heap = [4, 5]
Add 8: heap = [4, 5, 8]
Add 2: heap full, 2 < root(4), don't add

Final heap: [4, 5, 8] (root is 4, kth largest = 4)
```

### Adding more elements

```
add(3): heap full, 3 < 4, don't add → return 4
add(9): heap full, 9 > 4, replace → pop 4, push 9 → heap = [5, 8, 9] → return 5
```

## Complexity Analysis

| Operation | Time | Space |
|----------|------|-------|
| init | O(n log k) | O(k) |
| add | O(log k) | O(1) |

## Edge Cases

### k = 1
```rust
KthLargest::new(1, [])
add(5) → returns 5 (largest)
add(10) → returns 10
```
- Min-heap of size 1 = just the max element

### k = n (all elements)
```rust
KthLargest::new(3, [1, 2, 3])
add(4) → returns smallest (1st largest = 1)
```
- All elements stored

### Empty Initial
```rust
KthLargest::new(3, [])
add(5) → returns 5
add(4) → returns 4 (heap not full)
add(3) → returns 3 (heap not full)
```

## Why O(log k)?

BinaryHeap operations:
- Push: O(log n)
- Pop: O(log n)

With n = k (heap size), each add is O(log k).

## Common Mistakes

1. **Confusing min vs max heap**: kth largest needs min-heap (to get smallest of k largest)
2. **Not handling heap not full**: When heap has < k elements, root isn't kth largest yet
3. **Using regular BinaryHeap**: By default it's max-heap, need Reverse for min-heap

## Rust-Specific Patterns

1. **`BinaryHeap<Reverse<i32>>`**: Min-heap via Reverse wrapper
2. **`peek().unwrap().0`**: Get value from Reverse wrapper
3. **`drain(..)`**: Efficiently consume Vec in for loop