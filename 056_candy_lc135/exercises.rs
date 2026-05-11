/// Candy - LeetCode 135
/// Calculate minimum candies needed for children with ratings.

/// Approach: Two-pass greedy
/// Left to right pass ensures higher ratings get more candies than left neighbor.
/// Right to left pass ensures higher ratings get more candies than right neighbor.
/// Final candies = max(left_pass, right_pass) for each position.
pub fn candy(ratings: Vec<i32>) -> i32 {
    let n = ratings.len();
    if n == 0 { return 0; }

    let mut candies = vec![1; n];

    // Left to right pass
    for i in 1..n {
        if ratings[i] > ratings[i - 1] {
            candies[i] = candies[i - 1] + 1;
        }
    }

    // Right to left pass
    for i in (0..n - 1).rev() {
        if ratings[i] > ratings[i + 1] {
            candies[i] = candies[i + 1] + 1;
        }
    }

    candies.iter().sum()
}

/// Alternative: Single pass with peak tracking
pub fn candy_single_pass(ratings: Vec<i32>) -> i32 {
    let n = ratings.len();
    if n == 0 { return 0; }

    let mut total = 1;
    let mut up = 0;
    let mut down = 0;

    for i in 1..n {
        if ratings[i] >= ratings[i - 1] {
            down = 0;
            if ratings[i] == ratings[i - 1] {
                up = 1;
            } else {
                up += 1;
            }
            total += up;
        } else {
            up = 0;
            down += 1;
            total += down;
            if down >= up {
                total += 1;
            }
        }
    }

    total
}

/// Brute force approach for verification
pub fn candy_brute(ratings: Vec<i32>) -> i32 {
    let n = ratings.len();
    if n == 0 { return 0; }

    let mut candies = vec![1; n];

    loop {
        let mut changed = false;
        for i in 0..n {
            // Check left neighbor
            if i > 0 && ratings[i] > ratings[i - 1] && candies[i] <= candies[i - 1] {
                candies[i] = candies[i - 1] + 1;
                changed = true;
            }
            // Check right neighbor
            if i < n - 1 && ratings[i] > ratings[i + 1] && candies[i] <= candies[i + 1] {
                candies[i] = candies[i + 1] + 1;
                changed = true;
            }
        }
        if !changed { break; }
    }

    candies.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candy_basic() {
        assert_eq!(candy(vec![1, 0, 2]), 5);
    }

    #[test]
    fn test_candy_same_ratings() {
        assert_eq!(candy(vec![1, 1, 1]), 3);
    }

    #[test]
    fn test_candy_increasing() {
        assert_eq!(candy(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn test_candy_decreasing() {
        assert_eq!(candy(vec![5, 4, 3, 2, 1]), 15);
    }

    #[test]
    fn test_candy_single() {
        assert_eq!(candy(vec![1]), 1);
    }

    #[test]
    fn test_candy_empty() {
        assert_eq!(candy(vec![]), 0);
    }

    #[test]
    fn test_candy_two_children() {
        assert_eq!(candy(vec![1, 2]), 3);
    }

    #[test]
    fn test_candy_two_children_equal() {
        assert_eq!(candy(vec![2, 2]), 2);
    }

    #[test]
    fn test_candy_two_children_decrease() {
        assert_eq!(candy(vec![2, 1]), 2);
    }

    #[test]
    fn test_candy_v_shape() {
        // valley: high, low, high
        assert_eq!(candy(vec![5, 3, 4]), 5);
    }

    #[test]
    fn test_candy_peak() {
        // peak: low, high, low
        assert_eq!(candy(vec![1, 3, 1]), 3);
    }

    #[test]
    fn test_candy_complex() {
        assert_eq!(candy(vec![1, 2, 2]), 4);
    }

    #[test]
    fn test_candy_many_children() {
        let ratings = vec![1, 3, 4, 5, 2, 1];
        let result = candy(ratings.clone());
        assert!(result >= ratings.len() as i32);
    }

    #[test]
    fn test_candy_peak_then_decrease() {
        // [1, 3, 2] -> [2, 3, 1] = 6
        assert_eq!(candy(vec![1, 3, 2]), 5);
    }

    #[test]
    fn test_candy_increasing_then_flat() {
        assert_eq!(candy(vec![1, 2, 2, 3]), 5);
    }

    #[test]
    fn test_candy_multiple_peaks() {
        // Multiple equal peaks
        assert_eq!(candy(vec![1, 2, 1, 2, 1]), 7);
    }

    #[test]
    fn test_candy_two_pass_same_as_brute() {
        let ratings = vec![1, 0, 2];
        assert_eq!(candy(ratings.clone()), candy_brute(ratings));
    }

    #[test]
    fn test_candy_two_pass_same_as_brute_complex() {
        let ratings = vec![1, 3, 4, 5, 2, 1];
        assert_eq!(candy(ratings.clone()), candy_brute(ratings));
    }

    #[test]
    fn test_candy_single_pass_basic() {
        assert_eq!(candy_single_pass(vec![1, 0, 2]), 5);
    }

    #[test]
    fn test_candy_single_pass_same_as_two_pass() {
        let ratings = vec![1, 3, 4, 5, 2, 1];
        assert_eq!(candy_single_pass(ratings.clone()), candy(ratings));
    }

    #[test]
    fn test_candy_minimum_one_each() {
        let ratings = vec![0; 100];
        assert_eq!(candy(ratings), 100);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("056_candy_lc135 exercises - run tests with cargo test");
}
