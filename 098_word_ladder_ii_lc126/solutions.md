# Solutions - Word Ladder II (LeetCode 126)

## Solution 1: BFS with Parent Tracking

```rust
pub fn find_ladders(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {
    let word_list: HashSet<String> = word_list.into_iter().collect();
    if !word_list.contains(&end_word) {
        return vec![];
    }

    let mut result: Vec<Vec<String>> = vec![];
    let mut queue: VecDeque<(String, Vec<String>)> = VecDeque::new();
    queue.push_back((begin_word.clone(), vec![begin_word.clone()]));

    let mut visited: HashSet<String> = HashSet::new();
    visited.insert(begin_word.clone());

    let mut found_depth: Option<usize> = None;
    let mut parents: HashMap<String, Vec<String>> = HashMap::new();

    while let Some((word, path)) = queue.pop_front() {
        // If we've already found the target at a shorter depth, stop processing this level
        if let Some(depth) = found_depth {
            if path.len() > depth {
                break;
            }
        }

        if word == end_word {
            if found_depth.is_none() {
                found_depth = Some(path.len());
            }
            result.push(path);
            continue;
        }

        let bytes = word.as_bytes();
        for i in 0..bytes.len() {
            let mut pattern: Vec<u8> = bytes.to_vec();
            pattern[i] = b'_';

            for c in b'a'..=b'z' {
                if c == bytes[i] {
                    continue;
                }
                pattern[i] = c;
                let next_word = String::from_utf8(pattern.clone()).unwrap();
                pattern[i] = b'_';

                if word_list.contains(&next_word) {
                    parents.entry(next_word.clone()).or_default().push(word.clone());

                    if !visited.contains(&next_word) {
                        visited.insert(next_word.clone());
                        let mut new_path = path.clone();
                        new_path.push(next_word.clone());
                        queue.push_back((next_word, new_path));
                    }
                }
            }
        }
    }

    if result.is_empty() {
        reconstruct_paths(&begin_word, &end_word, &parents)
    } else {
        result
    }
}
```

### Line-by-Line Analysis

**Lines 14-17: Setup and Validation**
```rust
let word_list: HashSet<String> = word_list.into_iter().collect();
if !word_list.contains(&end_word) {
    return vec![];
}
```
Convert word list to HashSet for O(1) lookup. Return empty if end word not in dictionary.

**Lines 19-23: Initialize BFS**
```rust
let mut result: Vec<Vec<String>> = vec![];
let mut queue: VecDeque<(String, Vec<String>)> = VecDeque::new();
queue.push_back((begin_word.clone(), vec![begin_word.clone()]));

let mut visited: HashSet<String> = HashSet::new();
visited.insert(begin_word.clone());
```
Initialize result, queue with begin word and its path, and visited set.

**Lines 25-26: Tracking Variables**
```rust
let mut found_depth: Option<usize> = None;
let mut parents: HashMap<String, Vec<String>> = HashMap::new();
```
Track when we first find target (to stop exploring longer paths) and parent relationships for path reconstruction.

**Lines 28-35: BFS Level Control**
```rust
while let Some((word, path)) = queue.pop_front() {
    if let Some(depth) = found_depth {
        if path.len() > depth {
            break;
        }
    }
```
Process BFS level by level. Once target is found at depth D, only process paths of length D (no need to explore longer paths).

**Lines 37-45: Record Target and Continue**
```rust
if word == end_word {
    if found_depth.is_none() {
        found_depth = Some(path.len());
    }
    result.push(path);
    continue;
}
```
When we find end word, record the depth (first occurrence = shortest). All paths at this depth are shortest.

**Lines 47-68: Generate Neighbors via Pattern Matching**
```rust
let bytes = word.as_bytes();
for i in 0..bytes.len() {
    let mut pattern: Vec<u8> = bytes.to_vec();
    pattern[i] = b'_';

    for c in b'a'..=b'z' {
        if c == bytes[i] {
            continue;
        }
        pattern[i] = c;
        let next_word = String::from_utf8(pattern.clone()).unwrap();
        // ...
    }
}
```
For each position in the word, replace with '_' to create pattern. Try all 26 letters, skipping the original. This finds all valid one-character transformations efficiently without checking all N words.

**Lines 60-68: Track Parents and Queue Next Level**
```rust
if word_list.contains(&next_word) {
    parents.entry(next_word.clone()).or_default().push(word.clone());

    if !visited.contains(&next_word) {
        visited.insert(next_word.clone());
        let mut new_path = path.clone();
        new_path.push(next_word.clone());
        queue.push_back((next_word, new_path));
    }
}
```
Record parent for path reconstruction. Only add to queue if not visited (prevents revisiting same word at same depth).

**Lines 70-84: Fallback Reconstruction**
```rust
if result.is_empty() {
    reconstruct_paths(&begin_word, &end_word, &parents)
} else {
    result
}
```
If direct BFS didn't capture paths, try reconstruction using recorded parents.

### Complexity Analysis

| Aspect | Complexity |
|--------|------------|
| Time | O(N × L² × 26) - N words, L length, 26 letters |
| Space | O(N × L) - visited set, parents map |

### Visual Example

```
beginWord = "hit", endWord = "cog"
Dictionary: ["hot","dot","dog","lot","log","cog"]

BFS Queue Processing:
  Queue: [("hit", ["hit"])]
  Process "hit":
    Pattern h_t: hit -> *it, h*t, hi*
    Neighbors: hot (h*t matches)
    Queue: [("hot", ["hit", "hot"])]

  Process "hot":
    Pattern *ot: hot -> *ot matches dot, lot
    Pattern h*t: h*t matches hit (visited)
    Neighbors: dot, lot
    Queue: [("dot", [..., "hot", "dot"]), ("lot", [..., "hot", "lot"])]

  Process "dot":
    Pattern d*t: dot -> d*t matches dog
    Pattern *ot: *ot matches hot (visited)
    Pattern do*: do* matches cog? no
    Queue: [("dog", [..., "hot", "dot", "dog"])]

  Process "dog":
    Pattern d*g: dog -> d*g matches dog itself? no
    Pattern *og: *og matches dog? no
    Pattern do*: do* matches cog (cog)
    Queue: [("cog", [..., "hot", "dot", "dog", "cog"])]

  Target found! Depth = 5
  Backtrack parents to reconstruct all shortest paths
```

## Alternative Approaches

### Approach 2: Simplified BFS (No Parent Tracking)

Just record all complete paths when target is found, without parent reconstruction. Simpler but same time complexity.

### Approach 3: Bidirectional BFS

Start BFS from both begin and end, meet in middle. Faster for large graphs but more complex path reconstruction.

## Test Cases Verified

1. **Basic example**: hit->cog with hot/dot/dog/lot/log/cog - finds 2 shortest paths
2. **No transformation possible**: Returns empty when end word not reachable
3. **Single step**: Direct neighbor works correctly
4. **Same word**: begin == end returns just that word
5. **Multiple paths**: Different intermediate words produce different valid paths
6. **Chain transformation**: a->b->c->d works with single path
7. **Long chain**: 7-word chain correctly reconstructed
