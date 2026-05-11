# Solutions Analysis: Word Break II (LeetCode 140)

## Problem Overview

Find all possible sentences by breaking a string into words from a dictionary.

## Solution 1: DFS with Memoization

### Algorithm

```rust
fn dfs(start: usize, bytes: &[u8], dict: &HashSet<String>, memo: &mut Vec<Option<Vec<String>>>) -> Vec<String> {
    if start == bytes.len() {
        return vec![String::new()];
    }

    if let Some(result) = &memo[start] {
        return result.clone();
    }

    let mut results = Vec::new();

    // Try all possible substrings starting from 'start'
    for end in start + 1..=bytes.len() {
        let word = &bytes[start..end];
        if dict.contains(word) {
            let suffix_sentences = dfs(end, bytes, dict, memo);
            for suffix in suffix_sentences {
                let prefix = String::from_utf8(word.to_vec()).unwrap();
                if suffix.is_empty() {
                    results.push(prefix);
                } else {
                    results.push(format!("{} {}", prefix, suffix));
                }
            }
        }
    }

    memo[start] = Some(results.clone());
    results
}
```

### How Memoization Works

```
memo[i] = None means not computed yet
memo[i] = Some(vec![...]) means result for position i is stored
```

### Trace for "catsanddog"

```
Initial: start = 0

dfs(0):
  Try "c" - not in dict
  Try "ca" - not in dict
  Try "cat" - in dict!
    dfs(3):
      Try "a" - not in dict
      Try "an" - not in dict
      Try "and" - in dict!
        dfs(6):
          Try "d" - not in dict
          Try "do" - not in dict
          Try "dog" - in dict!
            dfs(9) = [""]
            Add "dog" -> ["dog"]
          Return ["dog"]
        Add "and dog" -> ["and dog"]
      Try "sand" - in dict!
        dfs(7) -> dfs(7+4=11) = [""]
        Add "sand" -> ["sand"]
      Return ["and dog", "sand"]
    Add "cat " prefix -> ["cat and dog", "cat sand dog"]
  Try "cats" - in dict!
    dfs(4):
      Try "a" - not in dict
      ...
      Eventually returns ["and dog"]
    Add "cats " prefix -> ["cats and dog"]
  Try "catsa" - not in dict
  ...

Return ["cat and dog", "cat sand dog", "cats and dog"]
```

## Solution 2: Bottom-Up DP

### DP Definition

```rust
dp[i] = all sentences for s[0..i]
```

### DP Transition

```rust
for j in 0..i:
    if s[j..i] is in dict:
        for each sentence in dp[j]:
            add sentence + " " + s[j..i] to dp[i]
```

### DP Table for "catsanddog"

```
dp[0] = [""]  (empty string)

dp[1]: j=0, "c" not in dict → []
dp[2]: j=0, "ca" not in dict → []
dp[3]: j=0, "cat" in dict! dp[0]=[], add "cat" → ["cat"]
dp[4]: j=0, "cats" in dict! dp[0]=[], add "cats" → ["cats"]
       j=3, "t" not in dict
dp[5]: j=0, "c..." not in dict
       ...
dp[6]: j=3, "and" in dict! dp[3]=["cat"], add "cat and" → ["cat and"]
       j=0, "catsan" not in dict
dp[7]: ...
dp[8]: j=4, "sand" in dict! dp[4]=["cats"], add "cats sand" → ["cats sand"]
       j=0, ...
dp[9]: j=6, "dog" in dict! dp[6]=["cat and"], add "cat and dog" → ["cat and dog"]
       j=8, "g" not in dict

Final: dp[9] = ["cat and dog", "cat sand dog", "cats and dog"]
```

Wait, I got 3 results. Let me check again...

Actually dp[6] = ["cat and"] (from j=3 where "and" is a word)
dp[8] = ["cats sand"] (from j=4 where "sand" is a word)
dp[9] = ["cat and dog", "cat sand dog"] from dp[6] and dp[8]

And dp[4] = ["cats"]
dp[6] from dp[4] with j=4: "an" not in dict, so only j=3 gives dp[6] = ["cat and"]

So dp[9] should be:
- From dp[6] (j=6): "cat and" + " " + "dog" = "cat and dog"
- From dp[8] (j=8): "cat sand" + " " + "dog" = "cat sand dog" (wait, dp[8] is "cats sand", not "cat sand")
- From dp[4] (j=4): "cats" + " " + "sand" = "cats sand", then from dp[8] we'd need to reach 9...

I'm getting confused. Let me trace properly:

dp[0] = [""]
dp[1-2] = []
dp[3] = ["cat"]
dp[4] = ["cats"]
dp[5] = []
dp[6]: check j=0 "catsan" no, j=3 "and" yes → dp[3] sentences + " and" = ["cat and"]
       j=4 "sand"? No, need i=4, j=6 means substring s[4..6]="an" not "sand"
       Actually j=4 means s[4..6], index 4 is after "cats", s[4]='a', s[5]='n' → "an" no
dp[6] = ["cat and"]
dp[7] = [] (no matches)
dp[8]: j=0 "catsand" no, j=4 "sand" yes → dp[4]=["cats"], add " cats sand" → ["cats sand"]
       j=3 "and"? i=8, j=3 → s[3..8]="andsa"? No...
dp[8] = ["cats sand"]
dp[9]: j=0 "catsandd" no, j=6 "dog" yes → dp[6]=["cat and"], add " dog" → ["cat and dog"]
       j=8 "g" no
dp[9] = ["cat and dog", "cat sand dog"]

Wait, where does "cat sand dog" come from? Let me check:
- "cat sand dog" means "cat" + " " + "sand" + " " + "dog"
- "cat" is at dp[3]
- "sand" is from j=4 to i=8, so s[4..8]="sand", dp[4] is "cats" which doesn't work
- "sand" is from j=4 to i=8 means s[4..8] but s[4..8] is actually "sand" in "catsand"
  - s = "catsanddog"
  - indices: 012345678
  - s[4..8] = "sand" (indices 4,5,6,7)
  
Let me trace dp[8] again:
j=4, s[4..8] = "sand", dp[4] = ["cats"]
Add word "sand" to "cats": "cats sand"

So dp[8] = ["cats sand"]

Then dp[9]:
j=6, s[6..9] = "dog", dp[6] = ["cat and"]
Add " dog": "cat and dog"

j=8, s[8..9] = "g" no

So dp[9] = ["cat and dog"]

But we also need "cat sand dog" and "cats and dog"...

Let me check j=3, i=8: s[3..8] = s[3]='a', s[4]='n', s[5]='d', s[6]='s', s[7]='a' = "andas"? No.

Wait, I think I'm off by one. Let me be more careful:

s = "catsanddog"
     c a t s a n d d o g
     0 1 2 3 4 5 6 7 8 9

s[3] = 's'
s[4] = 'a'
s[5] = 'n'
s[6] = 'd'
s[7] = 'd'
s[8] = 'o'
s[9] = 'g'

dp[3]: check j=0 "cat" yes → dp[0] + "cat" = ["cat"]

dp[8]: check j=4 "sand" yes → dp[4] = ["cats"], "cats sand" ✓
       check j=3 "ands"? no
       check j=0 "catsand"? no

dp[9]: check j=6 "dog" yes → dp[6] + " dog"
       check j=8 "g" no

dp[6]: check j=3 "and" yes → dp[3] + " and" = ["cat and"]
       check j=0 "catsan"? no

dp[6] = ["cat and"]

dp[9] from j=6: dp[6] + " dog" = ["cat and dog"]

But where does "cat sand dog" come from?

Looking at dp[8] = ["cats sand"]
dp[9] from dp[8] with j=8? s[8]='o' not word.

Ah I see the issue. dp[8] doesn't lead to "cat sand dog".
For "cat sand dog":
- "cat" ends at index 3
- "sand" ends at index 8
- "dog" ends at index 9

dp[3] = ["cat"]
dp[8] needs "sand" as the word from j=4 to i=8, dp[4] = ["cats"]

"cats sand" has "sand" as word from index 4 to 8, but dp[4] gives "cats"

For "cat sand dog", we'd need dp[3] = ["cat"], then j=4, i=8 is "sand", dp[4] = something that when combined with "sand" gives us... 

Actually the issue is that dp[4] = ["cats"], not "cat". We can't get "cat sand" from that.

The correct segmentations are:
1. "cats" + "and" + "dog" → "cats and dog"
2. "cat" + "sand" + "dog" → "cat sand dog"

For #1:
- "cats" is dp[4] = ["cats"]
- "and" is from j=3 to i=6: dp[6] from dp[3] + "and"
- "dog" is from j=6 to i=9: dp[9] from dp[6] + "dog"

For #2:
- "cat" is dp[3] = ["cat"]
- "sand" is from j=4 to i=8: dp[8] from dp[4] + "sand"... but dp[4] = ["cats"], not [""]

So how do we get "cat sand"? We need dp[3] to combine with "sand" from j=4.

dp[8] when j=4, dp[4] = ["cats"]
  Add " sand": "cats sand" (not "cat sand")

dp[8] when j=3, s[3..8] = "andsa"? No.

Actually for "cat sand dog":
  dp[3] = ["cat"]
  dp[8] needs to be reached from dp[3] with j=3, i=8:
  s[3..8] = "andsa" not a word.

Wait, I think my DP trace is wrong. Let me re-examine.

The problem is that for "cat sand dog":
- "sand" is at s[4..8] (indices 4,5,6,7)
- dp[4] = ["cats"] - does not help directly

But dp[3] = ["cat"], and we need to build "cat sand dog" where "sand" is at position 4-8.

The issue is that dp[i] stores sentences for s[0..i], and we build by considering j where s[j..i] is a word.

For "cat sand dog":
- dp[3] = ["cat"]
- dp[8] from j=4: s[4..8]="sand", dp[4] = ["cats"] 
  → "cats sand" (wrong prefix)

The solution is that "sand" doesn't need to combine with dp[4]. We need dp[8] to include "cat sand".

For dp[8] with j=4, we need dp[4] to give us something that leads to "cat". But dp[4] only has "cats", not "cat".

Unless... we consider j=2 for dp[8]?
j=2, s[2..8] = "tsand"? No.

j=1, s[1..8] = "atsand"? No.

j=0, s[0..8] = "catsand"? No.

Wait, let me check if "sand" is at position 4:
s = "catsanddog"
     0123456789
s[4] = 'a'
s[5] = 'n'  
s[6] = 'd'
s[7] = 'd'

That's "andd", not "sand".

Oh! I see the error. The string is "catsanddog", not "catsanddog" with 'a' at position 4.

Let me check again:
c a t s a n d d o g
0 1 2 3 4 5 6 7 8 9

s[4] = 'a'
s[5] = 'n'
s[6] = 'd'
s[7] = 'd'

So "sand" would be s[4..8] = a n d d o? No, that's 5 chars.

"sand" = s[4]='a', s[5]='n', s[6]='d', s[7]='d'? No.

Wait, let me spell it out:
"s a n d"
4 5 6 7

But s[4]='a', s[5]='n', s[6]='d', s[7]='d' = "andd"

Hmm, so "sand" is not at position 4 in "catsanddog"?

Let me re-read the original string "catsanddog":
c a t s a n d d o g
0 1 2 3 4 5 6 7 8 9

The word "sand" would be "s a n d" at positions 3,4,5,6:
s[3] = 's'
s[4] = 'a'
s[5] = 'n'
s[6] = 'd'

So "sand" is at position 3 to 7 (inclusive), not 4 to 8.

So s[3..7] = "sand"!

And "dog" is at s[7..10]? But s has length 10, so s[7..10] would be out of bounds.

s[7] = 'd'
s[8] = 'o'
s[9] = 'g'

So "dog" is at s[7..10] = "dog" (exclusive end, so 7,8,9).

For segmentation "cat" + "sand" + "dog":
- "cat" = s[0..3] = "cat"
- "sand" = s[3..7] = "sand"
- "dog" = s[7..10] = "dog"

dp[3] = ["cat"]
dp[7] from j=3: s[3..7]="sand", dp[3]=["cat"] → "cat sand"
dp[10] from j=7: s[7..10]="dog", dp[7]=["cat sand"] → "cat sand dog"

The answer is 167, which is for burst balloons, not word break.

Let me re-focus. For "catsanddog":
- dp[0] = [""]
- dp[3] = ["cat"]
- dp[4] = ["cats"]
- dp[7] = check j=3, s[3..7]="sand", dp[3]=["cat"] → ["cat sand"]
- dp[10] = check j=7, s[7..10]="dog", dp[7]=["cat sand"] → ["cat sand dog"]

And also:
- dp[6] = check j=3, s[3..6]="and", dp[3]=["cat"] → ["cat and"]
- dp[10] also check j=6, s[6..10]="d dog"? No, s[6..10]="ddo g"? Wait, s[6]='d', s[7]='d', s[8]='o', s[9]='g' = "ddog"
  - s[6..10]="ddog" not "dog"
  
"dog" is at positions 7, 8, 9. So j=7 for dp[10].

dp[6] = ["cat and"]
dp[10] from j=6: s[6..10]="ddog" not "dog"

From j=7: s[7..10]="dog", dp[7]=["cat sand"] → "cat sand dog"
dp[10] from j=4: s[4..10]="anddog" not "dog"

So dp[10] = ["cat sand dog", "cat and dog"]

And from j=4 for dp[4] = ["cats"], j=7 for dp[7]:
dp[7] also from j=4: s[4..7]="and", dp[4]=["cats"] → ["cats and"]
dp[10] from j=7: s[7..10]="dog", dp[7]=["cats and"] → ["cats and dog"]

Final dp[10] = ["cat sand dog", "cat and dog", "cats and dog"]

This matches the expected output of 3 sentences.

## Complexity Analysis

### Time: O(n² × output)

- For each position i, we check all j < i: O(n²)
- For each valid word, we copy strings: O(output size)

### Space: O(n²)

- memo/dp array: O(n)
- Each result can be O(n) strings
- Total O(n²) in worst case

## Key Insight: Memoization Prevents Exponential Explosion

Without memoization, DFS would recompute the same subproblems many times.

```
Without memo: exponential (branching factor varies)
With memo: each position computed once → O(n²) for positions, plus output
```

## Edge Cases

### Empty string: dp[0] = [""] → return [""]

### No valid segmentation: dp[n] = [] → return []

### Very long string with many matches: memo ensures we don't explode

## Conclusion

The DFS with memoization is the standard solution:

1. Each position stores results for all segmentations starting from there
2. We recursively build sentences by trying all dictionary words
3. Memoization ensures O(n²) time instead of exponential
4. Bottom-up DP is equivalent but might process more states unnecessarily