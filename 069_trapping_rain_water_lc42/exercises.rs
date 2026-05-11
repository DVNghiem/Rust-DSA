/// Trapping Rain Water - LeetCode 42
/// Calculate water trapped after rain using two-pointer approach.

/// Approach: Two-pointer O(n)
/// For each position, water = min(max_left, max_right) - height[i]
pub fn trap(height: Vec<i32>) -> i32 {
    let n = height.len();
    if n < 3 { return 0; }

    let mut left = 0;
    let mut right = n - 1;
    let mut left_max = height[left];
    let mut right_max = height[right];
    let mut water = 0;

    while left < right {
        if left_max < right_max {
            left += 1;
            left_max = left_max.max(height[left]);
            if left_max > height[left] {
                water += left_max - height[left];
            }
        } else {
            right -= 1;
            right_max = right_max.max(height[right]);
            if right_max > height[right] {
                water += right_max - height[right];
            }
        }
    }

    water
}

/// Approach 2: Dynamic Programming O(n)
/// Precompute max_left and max_right for each position
pub fn trap_dp(height: Vec<i32>) -> i32 {
    let n = height.len();
    if n < 3 { return 0; }

    let mut left_max = vec![0; n];
    let mut right_max = vec![0; n];

    left_max[0] = height[0];
    for i in 1..n {
        left_max[i] = left_max[i - 1].max(height[i]);
    }

    right_max[n - 1] = height[n - 1];
    for i in (0..n - 1).rev() {
        right_max[i] = right_max[i + 1].max(height[i]);
    }

    let mut water = 0;
    for i in 1..n - 1 {
        water += (left_max[i].min(right_max[i]) - height[i]).max(0);
    }

    water
}

/// Approach 3: Stack-based O(n)
pub fn trap_stack(height: Vec<i32>) -> i32 {
    let n = height.len();
    if n < 3 { return 0; }

    let mut stack: Vec<usize> = Vec::new();
    let mut water = 0;

    for i in 0..n {
        while let Some(&top) = stack.last() {
            if height[i] < height[top] {
                break;
            }
            stack.pop();
            if stack.is_empty() { break; }
            let distance = (i - stack.last().unwrap() - 1) as i32;
            let bounded_height = (height[i].min(height[*stack.last().unwrap()]) - height[top]) as i32;
            water += distance * bounded_height.max(0);
        }
        stack.push(i);
    }

    water
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trap_basic() {
        assert_eq!(trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]), 6);
    }

    #[test]
    fn test_trap_simple() {
        assert_eq!(trap(vec![4,2,0,3,2,5]), 9);
    }

    #[test]
    fn test_trap_no_water() {
        assert_eq!(trap(vec![1,2,3]), 0);
    }

    #[test]
    fn test_trap_empty() {
        assert_eq!(trap(vec![]), 0);
    }

    #[test]
    fn test_trap_two_elements() {
        assert_eq!(trap(vec![1,2]), 0);
    }

    #[test]
    fn test_trap_three_elements() {
        assert_eq!(trap(vec![1,0,2]), 1);
    }

    #[test]
    fn test_trap_valley() {
        assert_eq!(trap(vec![2,0,2]), 2);
    }

    #[test]
    fn test_trap_multiple_valleys() {
        assert_eq!(trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]), 6);
    }

    #[test]
    fn test_trap_dp_basic() {
        assert_eq!(trap_dp(vec![0,1,0,2,1,0,1,3,2,1,2,1]), 6);
    }

    #[test]
    fn test_trap_dp_simple() {
        assert_eq!(trap_dp(vec![4,2,0,3,2,5]), 9);
    }

    #[test]
    fn test_trap_stack_basic() {
        assert_eq!(trap_stack(vec![0,1,0,2,1,0,1,3,2,1,2,1]), 6);
    }

    #[test]
    fn test_trap_stack_simple() {
        assert_eq!(trap_stack(vec![4,2,0,3,2,5]), 9);
    }

    #[test]
    fn test_all_approaches_same() {
        let tests = vec![
            vec![0,1,0,2,1,0,1,3,2,1,2,1],
            vec![4,2,0,3,2,5],
            vec![1,2,3],
            vec![2,0,2],
            vec![3,0,2,0,4],
        ];
        for height in tests {
            let r1 = trap(height.clone());
            let r2 = trap_dp(height.clone());
            let r3 = trap_stack(height.clone());
            assert_eq!(r1, r2);
            assert_eq!(r2, r3);
        }
    }

    #[test]
    fn test_trap_flat() {
        assert_eq!(trap(vec![1,1,1,1]), 0);
    }

    #[test]
    fn test_trap_ascending() {
        assert_eq!(trap(vec![1,2,3,4,5]), 0);
    }

    #[test]
    fn test_trap_descending() {
        assert_eq!(trap(vec![5,4,3,2,1]), 0);
    }

    #[test]
    fn test_trap_one_water() {
        assert_eq!(trap(vec![0,2,0]), 0);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("069_trapping_rain_water_lc42 exercises - run tests with cargo test");
}
