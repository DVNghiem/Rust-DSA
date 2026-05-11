use std::collections::BinaryHeap;

// Definition for singly-linked list
#[derive(PartialEq, Debug, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }

    pub fn from_vec(v: Vec<i32>) -> Option<Box<ListNode>> {
        if v.is_empty() {
            return None;
        }
        let mut head = Box::new(ListNode::new(v[0]));
        let mut current = &mut head;
        for &val in v.iter().skip(1) {
            current.next = Some(Box::new(ListNode::new(val)));
            current = current.next.as_mut().unwrap();
        }
        Some(head)
    }

    pub fn to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut current = head;
        while let Some(node) = current {
            result.push(node.val);
            current = node.next;
        }
        result
    }
}

// Wrapper for BinaryHeap - only compare by val
#[derive(Clone, Debug)]
struct ListNodeWrapper(Option<Box<ListNode>>);

impl PartialEq for ListNodeWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_ref().map(|n| n.val) == other.0.as_ref().map(|n| n.val)
    }
}

impl Eq for ListNodeWrapper {}

impl PartialOrd for ListNodeWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ListNodeWrapper {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_val = self.0.as_ref().map(|n| n.val).unwrap_or(i32::MAX);
        let other_val = other.0.as_ref().map(|n| n.val).unwrap_or(i32::MAX);
        self_val.cmp(&other_val).reverse()
    }
}

/// Approach 1: Min-Heap
///
/// Use a min-heap to always extract the smallest head among all lists.
/// Time: O(N log k) where N = total nodes, k = number of lists
/// Space: O(k) for heap
pub fn mergeKLists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    // Min-heap: (value, list_node) - smallest value at top
    let mut heap: BinaryHeap<ListNodeWrapper> = BinaryHeap::new();

    // Push head of each non-empty list
    for list in lists {
        if list.is_some() {
            heap.push(ListNodeWrapper(list));
        }
    }

    // Build result list with dummy head
    let mut dummy = Box::new(ListNode::new(0));
    let mut tail = &mut dummy;

    // Extract min and push next from same list
    while let Some(ListNodeWrapper(Some(mut node))) = heap.pop() {
        // Take the next node before we move it
        let next = node.next.take();

        // Append to result
        tail.next = Some(node);
        tail = tail.next.as_mut().unwrap();

        // Push next from same list
        if next.is_some() {
            heap.push(ListNodeWrapper(next));
        }
    }

    dummy.next
}

/// Approach 2: Divide and Conquer (merge pairs recursively)
///
/// Pair up lists and merge them, then repeat until one remains.
/// Same time complexity O(N log k) but uses recursion.
pub fn mergeKLists_divide(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    if lists.is_empty() {
        return None;
    }

    fn mergeTwoLists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (Some(n), None) | (None, Some(n)) => Some(n),
            (Some(mut n1), Some(mut n2)) => {
                if n1.val <= n2.val {
                    let next = n1.next.take();
                    n1.next = mergeTwoLists(next, Some(n2));
                    Some(n1)
                } else {
                    let next = n2.next.take();
                    n2.next = mergeTwoLists(Some(n1), next);
                    Some(n2)
                }
            }
        }
    }

    fn divide(lists: &[Option<Box<ListNode>>]) -> Option<Box<ListNode>> {
        let len = lists.len();
        if len == 0 {
            None
        } else if len == 1 {
            lists[0].clone()
        } else {
            let mid = len / 2;
            let left = divide(&lists[..mid]);
            let right = divide(&lists[mid..]);
            mergeTwoLists(left, right)
        }
    }

    divide(&lists)
}

/// Helper function to merge two sorted lists (used by divide & conquer)
fn merge_two_lists(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (None, None) => None,
        (Some(n), None) | (None, Some(n)) => Some(n),
        (Some(mut n1), Some(mut n2)) => {
            if n1.val <= n2.val {
                n1.next = merge_two_lists(n1.next.take(), Some(n2));
                Some(n1)
            } else {
                n2.next = merge_two_lists(Some(n1), n2.next.take());
                Some(n2)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_merge() {
        // [[1,4,5],[1,3,4],[2,6]]
        // Expected: [1,1,2,3,4,4,5,6]
        let lists = vec![
            ListNode::from_vec(vec![1, 4, 5]),
            ListNode::from_vec(vec![1, 3, 4]),
            ListNode::from_vec(vec![2, 6]),
        ];
        let result = mergeKLists(lists);
        assert_eq!(ListNode::to_vec(result), vec![1, 1, 2, 3, 4, 4, 5, 6]);
    }

    #[test]
    fn test_empty_input() {
        let lists: Vec<Option<Box<ListNode>>> = vec![];
        let result = mergeKLists(lists);
        assert!(result.is_none());
    }

    #[test]
    fn test_single_list() {
        let lists = vec![ListNode::from_vec(vec![1, 2, 3])];
        let result = mergeKLists(lists);
        assert_eq!(ListNode::to_vec(result), vec![1, 2, 3]);
    }

    #[test]
    fn test_all_empty() {
        let lists: Vec<Option<Box<ListNode>>> = vec![None, None, None];
        let result = mergeKLists(lists);
        assert!(result.is_none());
    }

    #[test]
    fn test_single_element_per_list() {
        let lists = vec![
            ListNode::from_vec(vec![1]),
            ListNode::from_vec(vec![2]),
            ListNode::from_vec(vec![3]),
        ];
        let result = mergeKLists(lists);
        assert_eq!(ListNode::to_vec(result), vec![1, 2, 3]);
    }

    #[test]
    fn test_different_lengths() {
        let lists = vec![
            ListNode::from_vec(vec![1]),
            ListNode::from_vec(vec![2, 2, 2]),
            ListNode::from_vec(vec![1, 3, 5, 7]),
        ];
        let result = mergeKLists(lists);
        assert_eq!(ListNode::to_vec(result), vec![1, 1, 2, 2, 2, 3, 5, 7]);
    }

    #[test]
    fn test_duplicate_values() {
        let lists = vec![
            ListNode::from_vec(vec![1, 1, 1]),
            ListNode::from_vec(vec![1, 1, 1]),
            ListNode::from_vec(vec![1, 1, 1]),
        ];
        let result = mergeKLists(lists);
        assert_eq!(ListNode::to_vec(result), vec![1, 1, 1, 1, 1, 1, 1, 1, 1]);
    }

    #[test]
    fn test_already_sorted() {
        // Each list is already sorted, but they're all separate
        let lists = vec![
            ListNode::from_vec(vec![1]),
            ListNode::from_vec(vec![2]),
            ListNode::from_vec(vec![3]),
        ];
        let result = mergeKLists(lists);
        assert_eq!(ListNode::to_vec(result), vec![1, 2, 3]);
    }

    #[test]
    fn test_two_lists() {
        let lists = vec![
            ListNode::from_vec(vec![1, 3, 5, 7]),
            ListNode::from_vec(vec![2, 4, 6, 8]),
        ];
        let result = mergeKLists(lists);
        assert_eq!(ListNode::to_vec(result), vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_heap_vs_divide_same_result() {
        let lists = vec![
            ListNode::from_vec(vec![1, 4, 5]),
            ListNode::from_vec(vec![1, 3, 4]),
            ListNode::from_vec(vec![2, 6]),
        ];
        let heap_result = mergeKLists(lists.clone());
        let divide_result = mergeKLists_divide(lists);
        assert_eq!(ListNode::to_vec(heap_result), ListNode::to_vec(divide_result));
    }

    #[test]
    fn test_negative_values() {
        let lists = vec![
            ListNode::from_vec(vec![-3, -1, 2]),
            ListNode::from_vec(vec![-2, 0, 1]),
        ];
        let result = mergeKLists(lists);
        assert_eq!(ListNode::to_vec(result), vec![-3, -2, -1, 0, 1, 2]);
    }

    #[test]
    fn test_mixed_positive_negative() {
        let lists = vec![
            ListNode::from_vec(vec![-5, 1, 5]),
            ListNode::from_vec(vec![-10, -3, 3]),
        ];
        let result = mergeKLists(lists);
        assert_eq!(ListNode::to_vec(result), vec![-10, -5, -3, 1, 3, 5]);
    }

    #[test]
    fn test_large_k() {
        // Many lists with single elements
        let mut lists = Vec::new();
        for i in (0..100).rev() {
            lists.push(ListNode::from_vec(vec![i]));
        }
        let result = mergeKLists(lists);
        let vec = ListNode::to_vec(result);
        assert_eq!(vec.len(), 100);
        assert_eq!(vec, (0..100).collect::<Vec<_>>());
    }

    #[test]
    fn test_interleaved_lists() {
        // List 1: 1, 100
        // List 2: 50, 150
        // List 3: 25, 75, 125
        let lists = vec![
            ListNode::from_vec(vec![1, 100]),
            ListNode::from_vec(vec![50, 150]),
            ListNode::from_vec(vec![25, 75, 125]),
        ];
        let result = mergeKLists(lists);
        assert_eq!(ListNode::to_vec(result), vec![1, 25, 50, 75, 100, 125, 150]);
    }

    #[test]
    fn test_one_empty_list() {
        let lists = vec![
            ListNode::from_vec(vec![1, 2, 3]),
            None,
            ListNode::from_vec(vec![4, 5]),
        ];
        let result = mergeKLists(lists);
        assert_eq!(ListNode::to_vec(result), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_last_element_smallest() {
        let lists = vec![
            ListNode::from_vec(vec![5, 10, 15]),
            ListNode::from_vec(vec![1, 6, 11, 16]),
            ListNode::from_vec(vec![2, 7, 12]),
        ];
        let result = mergeKLists(lists);
        assert_eq!(ListNode::to_vec(result), vec![1, 2, 5, 6, 7, 10, 11, 12, 15, 16]);
    }

    #[test]
    fn test_alternating_small_large() {
        // List 1: all small values
        // List 2: all large values
        let lists = vec![
            ListNode::from_vec(vec![1, 3, 5, 7]),
            ListNode::from_vec(vec![2, 4, 6, 8]),
        ];
        let result = mergeKLists(lists);
        // Merged should interleave: 1,2,3,4,5,6,7,8
        assert_eq!(ListNode::to_vec(result), vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_divide_and_conquer_empty() {
        let lists: Vec<Option<Box<ListNode>>> = vec![];
        let result = mergeKLists_divide(lists);
        assert!(result.is_none());
    }

    #[test]
    fn test_divide_and_conquer_single() {
        let lists = vec![ListNode::from_vec(vec![1, 2, 3])];
        let result = mergeKLists_divide(lists);
        assert_eq!(ListNode::to_vec(result), vec![1, 2, 3]);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("030_merge_k_lists_lc23 exercises - run tests with cargo test");
}
