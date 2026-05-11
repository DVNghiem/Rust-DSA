/// Backtracking DFS approach
///
/// For each cell that matches word[0], start DFS to find the rest.
/// Mark visited cells to prevent reuse within the same path.
pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    if board.is_empty() || board[0].is_empty() {
        return word.is_empty();
    }

    let m = board.len();
    let n = board[0].len();
    let bytes = word.as_bytes();

    if bytes.is_empty() {
        return true;
    }

    if bytes.len() > m * n {
        return false;
    }

    // DFS with backtracking
    fn dfs(
        board: &[Vec<char>],
        bytes: &[u8],
        i: usize,
        j: usize,
        idx: usize,
        visited: &mut Vec<Vec<bool>>,
    ) -> bool {
        // Base case: matched all characters
        if idx == bytes.len() {
            return true;
        }

        // Bounds and character check
        if i >= board.len() || j >= board[0].len() {
            return false;
        }
        if visited[i][j] {
            return false;
        }
        if board[i][j] != bytes[idx] as char {
            return false;
        }

        // Mark as visited
        visited[i][j] = true;

        // Try all 4 directions
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        for (dr, dc) in directions.iter() {
            let ni = i as isize + dr;
            let nj = j as isize + dc;

            if ni >= 0 && ni < board.len() as isize && nj >= 0 && nj < board[0].len() as isize {
                if dfs(board, bytes, ni as usize, nj as usize, idx + 1, visited) {
                    return true;
                }
            }
        }

        // Backtrack: unmark
        visited[i][j] = false;

        false
    }

    // Try starting from each cell
    let mut visited = vec![vec![false; n]; m];
    for i in 0..m {
        for j in 0..n {
            if board[i][j] == bytes[0] as char {
                visited[i][j] = true;
                if dfs(&board, bytes, i, j, 1, &mut visited) {
                    return true;
                }
                visited[i][j] = false;
            }
        }
    }

    false
}

/// Alternative: In-place modification approach
/// Instead of visited array, mark with a sentinel character
pub fn exist_inplace(board: Vec<Vec<char>>, word: String) -> bool {
    if board.is_empty() || board[0].is_empty() {
        return word.is_empty();
    }

    let m = board.len();
    let n = board[0].len();
    let bytes = word.as_bytes();

    if bytes.is_empty() {
        return true;
    }

    fn dfs(
        board: &mut Vec<Vec<char>>,
        bytes: &[u8],
        i: usize,
        j: usize,
        idx: usize,
    ) -> bool {
        if idx == bytes.len() {
            return true;
        }

        if i >= board.len() || j >= board[0].len() {
            return false;
        }
        if board[i][j] != bytes[idx] as char {
            return false;
        }

        // Mark: use a special char that won't match any letter
        let temp = board[i][j];
        board[i][j] = '#'; // Sentinel

        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        for (dr, dc) in directions.iter() {
            let ni = i as isize + dr;
            let nj = j as isize + dc;

            if ni >= 0 && ni < board.len() as isize && nj >= 0 && nj < board[0].len() as isize {
                if dfs(board, bytes, ni as usize, nj as usize, idx + 1) {
                    return true;
                }
            }
        }

        board[i][j] = temp; // Restore

        false
    }

    for i in 0..m {
        for j in 0..n {
            if board[i][j] == bytes[0] as char {
                let mut board = board.clone();
                if dfs(&mut board, bytes, i, j, 1) {
                    return true;
                }
            }
        }
    }

    false
}

/// Optimized version with early pruning
pub fn exist_optimized(board: Vec<Vec<char>>, word: String) -> bool {
    if board.is_empty() || board[0].is_empty() {
        return word.is_empty();
    }

    let m = board.len();
    let n = board[0].len();
    let bytes = word.as_bytes();

    if bytes.is_empty() {
        return true;
    }

    // Count frequency of each character in board and word
    // If any character in word appears more times in word than in board, impossible
    fn can_form(board: &[Vec<char>], word: &[u8]) -> bool {
        let mut board_count = [0i32; 256];
        let mut word_count = [0i32; 256];

        for row in board {
            for &c in row {
                board_count[c as usize] += 1;
            }
        }
        for &c in word {
            word_count[c as usize] += 1;
        }

        for i in 0..256 {
            if word_count[i] > board_count[i] {
                return false;
            }
        }
        true
    }

    if !can_form(&board, bytes) {
        return false;
    }

    fn dfs(
        board: &[Vec<char>],
        bytes: &[u8],
        i: usize,
        j: usize,
        idx: usize,
        visited: &mut Vec<Vec<bool>>,
    ) -> bool {
        if idx == bytes.len() {
            return true;
        }

        if i >= board.len() || j >= board[0].len() {
            return false;
        }
        if visited[i][j] {
            return false;
        }
        if board[i][j] != bytes[idx] as char {
            return false;
        }

        visited[i][j] = true;

        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        for (dr, dc) in directions.iter() {
            let ni = i as isize + dr;
            let nj = j as isize + dc;

            if ni >= 0 && ni < board.len() as isize && nj >= 0 && nj < board[0].len() as isize {
                if dfs(board, bytes, ni as usize, nj as usize, idx + 1, visited) {
                    return true;
                }
            }
        }

        visited[i][j] = false;

        false
    }

    let mut visited = vec![vec![false; n]; m];
    for i in 0..m {
        for j in 0..n {
            if board[i][j] == bytes[0] as char {
                visited[i][j] = true;
                if dfs(&board, bytes, i, j, 1, &mut visited) {
                    return true;
                }
                visited[i][j] = false;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_board(v: Vec<&str>) -> Vec<Vec<char>> {
        v.iter().map(|s| s.chars().collect()).collect()
    }

    #[test]
    fn test_basic_exist() {
        let board = create_board(vec!["ABCE", "SFCS", "ADEE"]);
        assert!(exist(board, "ABCCED".to_string()));
    }

    #[test]
    fn test_see_exists() {
        let board = create_board(vec!["ABCE", "SFCS", "ADEE"]);
        assert!(exist(board, "SEE".to_string()));
    }

    #[test]
    fn test_abcb_not_exist() {
        let board = create_board(vec!["ABCE", "SFCS", "ADEE"]);
        assert!(!exist(board, "ABCB".to_string()));
    }

    #[test]
    fn test_empty_word() {
        let board = create_board(vec!["A"]);
        assert!(exist(board, "".to_string()));
    }

    #[test]
    fn test_empty_board() {
        let board: Vec<Vec<char>> = vec![];
        assert!(!exist(board, "A".to_string()));
        assert!(exist(board, "".to_string()));
    }

    #[test]
    fn test_single_cell_match() {
        let board = create_board(vec!["A"]);
        assert!(exist(board, "A".to_string()));
    }

    #[test]
    fn test_single_cell_no_match() {
        let board = create_board(vec!["A"]);
        assert!(!exist(board, "B".to_string()));
    }

    #[test]
    fn test_word_needs_backtrack() {
        // Word: "AB" - A at (0,0), B at (0,1)
        let board = create_board(vec!["AB"]);
        assert!(exist(board, "AB".to_string()));
    }

    #[test]
    fn test_word_longer_than_board() {
        let board = create_board(vec!["A"]);
        assert!(!exist(board, "AA".to_string()));
    }

    #[test]
    fn test_all_same_letter() {
        let board = create_board(vec!["AAAA"]);
        assert!(exist(board, "AAAA".to_string()));
        assert!(!exist(board, "AAAAA".to_string()));
    }

    #[test]
    fn test_diagonal_not_allowed() {
        // A at (0,0) and B at (1,1) - diagonal not adjacent
        let board = create_board(vec!["AB", "CD"]);
        assert!(!exist(board, "AD".to_string())); // A→D not adjacent
        assert!(exist(board, "AC".to_string())); // A→C adjacent
    }

    #[test]
    fn test_cannot_reuse_cell() {
        // Board: "AA" - word: "AAA"
        // First A at (0,0), second A at (0,1), third A... no more
        let board = create_board(vec!["AA"]);
        assert!(!exist(board, "AAA".to_string()));
    }

    #[test]
    fn test_longer_word() {
        let board = create_board(vec!["AB", "CD"]);
        assert!(exist(board, "ABCD".to_string()));
    }

    #[test]
    fn test_backtracking_needed() {
        // Board where path must backtrack
        let board = create_board(vec!["ABC", "DEF", "GHI"]);
        // "ADG" - A at (0,0), D at (1,0), G at (2,0)
        assert!(exist(board, "ADG".to_string()));
    }

    #[test]
    fn test_all_visited() {
        let board = create_board(vec!["ABC"]);
        assert!(exist(board, "ABC".to_string()));
        assert!(exist(board, "AB".to_string()));
        assert!(exist(board, "BC".to_string()));
    }

    #[test]
    fn test_optimized_same_as_basic() {
        let board = create_board(vec!["ABCE", "SFCS", "ADEE"]);
        let basic = exist(board.clone(), "ABCCED".to_string());
        let optimized = exist_optimized(board, "ABCCED".to_string());
        assert_eq!(basic, optimized);
    }

    #[test]
    fn test_pruning_optimization() {
        // Word has more 'X' than board contains
        let board = create_board(vec!["AB"]);
        // Board has 0 'X's, word needs 1 'X' - should return false quickly
        let result = exist_optimized(board, "AX".to_string());
        assert!(!result);
    }

    #[test]
    fn test_inplace_same_as_visited() {
        let board = create_board(vec!["ABCE", "SFCS", "ADEE"]);
        let visited_result = exist(board.clone(), "SEE".to_string());
        let inplace_result = exist_inplace(board, "SEE".to_string());
        assert_eq!(visited_result, inplace_result);
    }

    #[test]
    fn test_complex_path() {
        let board = create_board(vec!["ABCDEFG"]);
        // Can only go right
        assert!(exist(board, "ABCDEFG".to_string()));
        assert!(!exist(board, "ACDEF".to_string())); // Skip B
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("Word Search exercises - run tests with cargo test");
}