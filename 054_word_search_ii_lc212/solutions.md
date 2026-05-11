# Word Search II Solution - LeetCode 212 (Complete)

## Solution Analysis

### Trie + DFS Approach

```rust
struct TrieNode {
    children: HashMap<char, TrieNode>,
    word: Option<String>,
}

pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
    // Build Trie from all words
    let mut trie = Trie::new();
    for word in &words {
        trie.insert(word);
    }

    let mut result = HashSet::new();
    let m = board.len();
    let n = board[0].len();
    let mut visited = vec![vec![false; n]; m];

    fn dfs(row: usize, col: usize, board: &Vec<Vec<char>>, node: &mut TrieNode,
           visited: &mut Vec<Vec<bool>>, result: &mut HashSet<String>) {
        let ch = board[row][col];
        if let Some(next) = node.children.get_mut(&ch) {
            if let Some(word) = next.word.take() {
                result.insert(word);
            }

            visited[row][col] = true;
            let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

            for (dr, dc) in directions.iter() {
                let new_row = row as i32 + dr;
                let new_col = col as i32 + dc;
                if new_row >= 0 && new_row < board.len() as i32
                    && new_col >= 0 && new_col < board[0].len() as i32
                    && !visited[new_row as usize][new_col as usize]
                {
                    dfs(new_row as usize, new_col as usize, board, next, visited, result);
                }
            }
            visited[row][col] = false;
        }
    }

    // Start DFS from each cell
    for i in 0..m {
        for j in 0..n {
            dfs(i, j, &board, &mut trie.root, &mut visited, &mut result);
        }
    }

    result.into_iter().collect()
}
```

## Why Trie for This Problem

**Naive approach**: Run DFS for each word
- Time: O(m × n × word_length × num_words)
- Expensive when many words

**Trie approach**: Build Trie once, then single DFS explores all paths
- Time: O(m × n × 4^L) where L is max word length
- Shares exploration across all words

## Key Insight: Pruning via Trie

When exploring a path, if we reach a node with no children for current character, we stop immediately. This provides significant pruning.

## Why Mark word.take()?

Once a word is found, we remove it from the Trie to avoid finding it again (preventing duplicates in result). This is an optimization.

## Complexity Analysis

| Operation | Time | Space |
|-----------|------|-------|
| Build Trie | O(total_chars) | O(total_chars) |
| DFS per cell | O(m × n × 4^L) | O(L) stack |
| Total | O(m × n + total_chars) | O(total_chars) |

## Edge Cases

1. **Empty board**: Return `[]`
2. **Empty words**: Return `[]`
3. **Single cell**: Works correctly
4. **No valid words**: Return `[]`

## Common Mistakes

1. **Not marking visited**: Would allow reusing cells in same word
2. **Not unmarking on backtrack**: Would prevent using cell in other paths
3. **Not removing found words**: Would cause duplicates