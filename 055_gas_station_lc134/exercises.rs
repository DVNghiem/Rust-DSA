/// Gas Station - LeetCode 134
/// Find the starting station that can complete a circular tour.

/// Approach 1: Greedy (Optimal O(n))
/// If total gas >= total cost, solution exists. When we fail at station i,
/// no station between current start and i can be the solution.
pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let mut total_tank = 0;
    let mut curr_tank = 0;
    let mut start = 0;

    for i in 0..gas.len() {
        total_tank += gas[i] - cost[i];
        curr_tank += gas[i] - cost[i];
        if curr_tank < 0 {
            start = (i + 1) as i32;
            curr_tank = 0;
        }
    }

    if total_tank >= 0 { start } else { -1 }
}

/// Approach 2: Brute Force O(n²)
pub fn can_complete_circuit_brute(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    for start in 0..gas.len() {
        let mut tank = 0;
        for i in 0..gas.len() {
            let idx = (start + i) % gas.len();
            tank += gas[idx] - cost[idx];
            if tank < 0 { break; }
        }
        if tank >= 0 { return start as i32; }
    }
    -1
}

/// Helper function to compute total net gas
pub fn total_net(gas: &[i32], cost: &[i32]) -> i32 {
    gas.iter().zip(cost.iter()).map(|(g, c)| g - c).sum()
}

/// Check if a starting position works
fn check_start(gas: &[i32], cost: &[i32], start: usize) -> bool {
    let mut tank = 0;
    for i in 0..gas.len() {
        let idx = (start + i) % gas.len();
        tank += gas[idx] - cost[idx];
        if tank < 0 { return false; }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_complete_circuit_basic() {
        assert_eq!(can_complete_circuit(vec![1,2,3,4,5], vec![3,4,5,1,2]), 3);
    }

    #[test]
    fn test_can_complete_circuit_no_solution() {
        assert_eq!(can_complete_circuit(vec![2,3,4], vec![3,4,5]), -1);
    }

    #[test]
    fn test_can_complete_circuit_start_zero() {
        assert_eq!(can_complete_circuit(vec![2, 3, 4], vec![2, 3, 4]), 0);
    }

    #[test]
    fn test_can_complete_circuit_single_station_valid() {
        assert_eq!(can_complete_circuit(vec![5], vec![2]), 0);
    }

    #[test]
    fn test_can_complete_circuit_single_station_invalid() {
        assert_eq!(can_complete_circuit(vec![1], vec![2]), -1);
    }

    #[test]
    fn test_can_complete_circuit_single_station_equal() {
        assert_eq!(can_complete_circuit(vec![5], vec![5]), 0);
    }

    #[test]
    fn test_can_complete_circuit_two_stations_valid() {
        assert_eq!(can_complete_circuit(vec![1, 2], vec![2, 1]), 1);
    }

    #[test]
    fn test_can_complete_circuit_two_stations_first_valid() {
        assert_eq!(can_complete_circuit(vec![2, 1], vec![1, 2]), 0);
    }

    #[test]
    fn test_can_complete_circuit_two_stations_no_solution() {
        assert_eq!(can_complete_circuit(vec![1, 1], vec![2, 2]), -1);
    }

    #[test]
    fn test_can_complete_circuit_all_greater() {
        let gas = vec![5, 1, 2, 3, 4];
        let cost = vec![3, 4, 5, 1, 2];
        assert_eq!(can_complete_circuit(gas, cost), 4);
    }

    #[test]
    fn test_can_complete_circuit_negative_diffs() {
        assert_eq!(can_complete_circuit(vec![1, 2, 3], vec![2, 3, 4]), -1);
    }

    #[test]
    fn test_can_complete_circuit_zero_costs() {
        assert_eq!(can_complete_circuit(vec![1, 2, 3], vec![0, 0, 0]), 0);
    }

    #[test]
    fn test_can_complete_circuit_all_equal() {
        assert_eq!(can_complete_circuit(vec![5, 5, 5], vec![5, 5, 5]), 0);
    }

    #[test]
    fn test_can_complete_circuit_alternating() {
        // gas = [1, 3, 1], cost = [2, 2, 2]
        // net: -1, +1, -1 = -1 total, no solution
        assert_eq!(can_complete_circuit(vec![1, 3, 1], vec![2, 2, 2]), -1);
    }

    #[test]
    fn test_can_complete_circuit_large_numbers() {
        assert_eq!(can_complete_circuit(vec![1000000], vec![999999]), 0);
    }

    #[test]
    fn test_can_complete_circuit_large_numbers_fail() {
        assert_eq!(can_complete_circuit(vec![1000000], vec![1000001]), -1);
    }

    #[test]
    fn test_greedy_and_brute_same_result() {
        let gas = vec![1, 2, 3, 4, 5];
        let cost = vec![3, 4, 5, 1, 2];
        assert_eq!(can_complete_circuit(gas.clone(), cost.clone()),
                   can_complete_circuit_brute(gas, cost));
    }

    #[test]
    fn test_greedy_and_brute_same_no_solution() {
        let gas = vec![2, 3, 4];
        let cost = vec![3, 4, 5];
        assert_eq!(can_complete_circuit(gas.clone(), cost.clone()),
                   can_complete_circuit_brute(gas, cost));
    }

    #[test]
    fn test_total_net_positive() {
        assert!(total_net(&[1, 2, 3], &[1, 1, 1]) > 0);
    }

    #[test]
    fn test_total_net_negative() {
        assert!(total_net(&[1, 1], &[2, 2]) < 0);
    }

    #[test]
    fn test_total_net_zero() {
        assert_eq!(total_net(&[1, 2], &[2, 1]), 0);
    }

    #[test]
    fn test_check_start_valid() {
        assert!(check_start(&[1, 2, 3, 4, 5], &[3, 4, 5, 1, 2], 3));
    }

    #[test]
    fn test_check_start_invalid() {
        assert!(!check_start(&[1, 2, 3], &[2, 3, 4], 0));
    }

    #[test]
    fn test_circuit_with_negative_at_start() {
        // gas = [3, 1, 1], cost = [1, 2, 2]
        // net: +2, -1, -1 = 0, start should be 0
        assert_eq!(can_complete_circuit(vec![3, 1, 1], vec![1, 2, 2]), 0);
    }

    #[test]
    fn test_circuit_middle_start() {
        // gas = [1, 2, 3, 4, 5], cost = [5, 4, 3, 2, 1]
        // net: -4, -2, 0, +2, +4 = 0
        // Best start is 2 (or any that works)
        let result = can_complete_circuit(vec![1, 2, 3, 4, 5], vec![5, 4, 3, 2, 1]);
        assert!(check_start(&[1, 2, 3, 4, 5], &[5, 4, 3, 2, 1], result as usize));
    }

    #[test]
    fn test_many_stations() {
        let gas: Vec<i32> = vec![1; 100];
        let cost: Vec<i32> = vec![1; 100];
        assert_eq!(can_complete_circuit(gas, cost), 0);
    }

    #[test]
    fn test_circuit_almost_fail() {
        // Only one valid start in many
        let gas = vec![2, 3, 4, 5, 1];
        let cost = vec![3, 4, 5, 1, 2];
        assert_eq!(can_complete_circuit(gas, cost), 2);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("055_gas_station_lc134 exercises - run tests with cargo test");
}
