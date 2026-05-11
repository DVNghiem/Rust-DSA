//! Exercises for Reconstruct Itinerary (LeetCode 332)
//!
//! # Topics Covered
//! - Eulerian path
//! - Hierholzer's algorithm
//! - Graph traversal
//! - Path reconstruction
//! - DFS
//!
//! # Difficulty: Hard

use std::collections::{HashMap, BTreeMap};

/// Hierholzer's algorithm approach using BTreeMap for automatic sorting
/// Returns lexicographically smallest itinerary starting from JFK
pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
    let mut graph: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for ticket in &tickets {
        let from = ticket[0].clone();
        let to = ticket[1].clone();
        graph.entry(from).or_default().push(to);
    }

    // Sort destinations for each airport to get lexical order
    for dests in graph.values_mut() {
        dests.sort();
    }

    let mut result: Vec<String> = vec![];
    let mut stack: Vec<String> = vec!["JFK".to_string()];

    while let Some(airport) = stack.pop() {
        if let Some(next) = graph.get_mut(&airport).and_then(|dests| dests.pop()) {
            // Push current airport back (we'll process it after its children)
            stack.push(airport);
            stack.push(next);
        } else {
            // No more destinations, add to result
            result.push(airport);
        }
    }

    result.reverse();
    result
}

/// Verify if an itinerary uses all tickets exactly once
pub fn is_valid_itinerary(tickets: Vec<Vec<String>>, itinerary: Vec<String>) -> bool {
    if itinerary.is_empty() || itinerary[0] != "JFK" {
        return false;
    }

    let mut ticket_count: HashMap<String, i32> = HashMap::new();
    for ticket in &tickets {
        let key = format!("{}->{}", ticket[0], ticket[1]);
        *ticket_count.entry(key).or_insert(0) += 1;
    }

    // Check we use each ticket exactly once
    for i in 0..itinerary.len().saturating_sub(1) {
        let key = format!("{}->{}", itinerary[i], itinerary[i + 1]);
        if let Some(count) = ticket_count.get_mut(&key) {
            if *count <= 0 {
                return false;
            }
            *count -= 1;
        } else {
            return false;
        }
    }

    // All tickets must be used
    for count in ticket_count.values() {
        if *count != 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_example() {
        let tickets = vec![
            vec!["JFK".to_string(), "MUC".to_string()],
            vec!["MUC".to_string(), "LHR".to_string()],
            vec!["LHR".to_string(), "SFO".to_string()],
        ];
        let result = find_itinerary(tickets);
        assert_eq!(result[0], "JFK");
        assert_eq!(result.len(), 4);
        assert_eq!(result, vec!["JFK", "MUC", "LHR", "SFO"]);
    }

    #[test]
    fn test_single_ticket() {
        let tickets = vec![vec!["JFK".to_string(), "SFO".to_string()]];
        let result = find_itinerary(tickets);
        assert_eq!(result, vec!["JFK", "SFO"]);
    }

    #[test]
    fn test_two_tickets() {
        let tickets = vec![
            vec!["JFK".to_string(), "MUC".to_string()],
            vec!["MUC".to_string(), "LHR".to_string()],
        ];
        let result = find_itinerary(tickets);
        assert_eq!(result, vec!["JFK", "MUC", "LHR"]);
    }

    #[test]
    fn test_valid_itinerary_check() {
        let tickets = vec![
            vec!["JFK".to_string(), "SFO".to_string()],
            vec!["SFO".to_string(), "MUC".to_string()],
        ];
        let valid = vec!["JFK".to_string(), "SFO".to_string(), "MUC".to_string()];
        assert!(is_valid_itinerary(tickets.clone(), valid));
    }

    #[test]
    fn test_invalid_itinerary_wrong_start() {
        let tickets = vec![
            vec!["JFK".to_string(), "SFO".to_string()],
        ];
        let invalid = vec!["SFO".to_string(), "JFK".to_string()];
        assert!(!is_valid_itinerary(tickets, invalid));
    }

    #[test]
    fn test_invalid_itinerary_unused_ticket() {
        let tickets = vec![
            vec!["JFK".to_string(), "SFO".to_string()],
            vec!["SFO".to_string(), "MUC".to_string()],
        ];
        let invalid = vec!["JFK".to_string(), "SFO".to_string()];
        assert!(!is_valid_itinerary(tickets, invalid));
    }

    #[test]
    fn test_empty_tickets() {
        let tickets: Vec<Vec<String>> = vec![];
        let result = find_itinerary(tickets);
        assert_eq!(result, vec!["JFK"]);
    }

    #[test]
    fn test_two_hops_roundtrip() {
        let tickets = vec![
            vec!["JFK".to_string(), "ATL".to_string()],
            vec!["ATL".to_string(), "LAX".to_string()],
            vec!["LAX".to_string(), "ATL".to_string()],
        ];
        let result = find_itinerary(tickets);
        assert_eq!(result, vec!["JFK", "ATL", "LAX", "ATL"]);
    }

    #[test]
    fn test_must_start_from_jfk() {
        let tickets = vec![
            vec!["AAA".to_string(), "BBB".to_string()],
            vec!["BBB".to_string(), "CCC".to_string()],
        ];
        let result = find_itinerary(tickets);
        assert_eq!(result[0], "JFK");
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("097_reconstruct_itinerary_lc332 exercises - run tests with cargo test");
}
