# Rust DSA Learning Roadmap

A comprehensive, hands-on Data Structures and Algorithms course using Rust. Master DSA through 100 curated problems with real LeetCode exercises.

## Overview

This repository contains 100 focused learning modules covering essential DSA topics. Each module provides:
- **Theory**: Deep dive into concepts with visual explanations
- **Exercises**: Real LeetCode-style problems to solve
- **Solutions**: Detailed line-by-line analysis

## Structure

```
rust-dsa/
├── 001_big_o_analysis/           # Complexity analysis foundations
├── 002_two_sum_lc1/             # Two Sum (LeetCode #1)
├── 003_contains_duplicate_lc26/  # Contains Duplicate (#26)
├── ...
└── 100_lru_cache_advanced_lc146/ # Advanced LRU Cache
```

Each folder contains:
| File | Description |
|------|-------------|
| `README.md` | Theory + exercise specifications (1500+ words) |
| `Cargo.toml` | Rust project configuration |
| `exercises.rs` | Starter code with `todo!()` implementations |
| `solutions.md` | Complete solution analysis (2000+ words) |

## Difficulty Progression

| Range | Level | Topics |
|-------|-------|--------|
| **001-050** | ★☆☆ Easy | Arrays, Hashing, Linked Lists, Stacks, Trees, Graphs, Basic DP |
| **051-085** | ★★☆ Medium | Backtracking, Greedy, System Design, Advanced DP |
| **086-100** | ★★★ Hard | Tries, Advanced Data Structures, Competitive Problems |

## Topic Map

### Foundations (001-025)
- Big O Analysis, Arrays, Hashing, Two Pointers
- Linked Lists, Stacks, Queues

### Intermediate (026-050)
- Trees, Binary Search Trees, Heaps
- Graphs (BFS/DFS), Dynamic Programming

### Advanced (051-075)
- Backtracking, Greedy Algorithms
- System Design Problems, Tries

### Expert (076-100)
- Advanced Data Structures, String Algorithms
- Hard LeetCode Problems, Optimization

## Quick Start

```bash
# Navigate to any problem folder
cd 002_two_sum_lc1

# Run tests to see what needs to be implemented
cargo test

# Implement the todo!() placeholders in exercises.rs
# Then run tests again to verify your solution
cargo test
```

## Example Workflow

```bash
# 1. Pick a topic
cd 011_three_sum_lc15

# 2. Read the theory
cat README.md

# 3. Run tests to see expected behavior
cargo test

# 4. Implement solution in exercises.rs
# (Replace todo!() with your code)

# 5. Verify your solution
cargo test
```

## Topics Covered

- **Arrays & Hashing**: Two Sum, Contains Duplicate, Valid Anagram, Group Anagrams, Top K Frequent, Product Except Self
- **Two Pointers**: Container With Most Water, Valid Palindrome, Three Sum
- **Linked Lists**: Reverse, Merge, Cycle Detection, Remove Nth
- **Stacks & Queues**: Valid Parentheses, Min Stack, Daily Temperatures, RPN, Implement Queue
- **Trees**: Inorder/Preorder/Postorder, Invert, Max Depth, Diameter, BST Validation, LCA, Kth Smallest
- **Graphs**: Number of Islands, Rotting Oranges, Word Search, Course Schedule, Clone Graph, Word Ladder
- **Dynamic Programming**: Climbing Stairs, House Robber, Coin Change, LIS, Decode Ways, Unique Paths, Edit Distance
- **Advanced**: Trie, LRU Cache, Median Finder, Sliding Window Maximum

## Test Results

To see which tests pass without implementation:

```bash
# Check all folders
for d in 0*/; do
  echo "=== $(basename $d) ==="
  cargo test 2>&1 | grep -E "(test result|FAILED|passed)"
done
```

## Solutions

Each `solutions.md` contains:
- Multiple approaches per problem
- Time/Space complexity analysis
- Line-by-line code walkthrough
- Common pitfalls and edge cases
- Rust-specific idioms and patterns

## Requirements

- Rust 1.56+ (edition 2021)
- Standard library only (no external crates)

## Contributing

1. Pick an unimplemented problem
2. Read README.md for specifications
3. Implement in exercises.rs
4. Write tests if needed
5. Compare with solutions.md

## License

MIT License - Use freely for learning.