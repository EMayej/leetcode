/**
 *
 * https://leetcode.com/problems/merge-k-sorted-lists
 *
 * 23. Merge k Sorted Lists
 *
 * You are given an array of `k` linked-lists `lists`, each linked-list is sorted
 * in ascending order.
 *
 * *Merge all the linked-lists into one sorted linked-list and return it.*
 *
 *
 * **Example 1:**
 *
 * **Input:** lists = [[1,4,5],[1,3,4],[2,6]]
 * **Output:** [1,1,2,3,4,4,5,6]
 * **Explanation:** The linked-lists are:
 * [
 *   1->4->5,
 *   1->3->4,
 *   2->6
 * ]
 * merging them into one sorted list:
 * 1->1->2->3->4->4->5->6
 *
 * **Example 2:**
 *
 * **Input:** lists = []
 * **Output:** []
 *
 * **Example 3:**
 *
 * **Input:** lists = [[]]
 * **Output:** []
 *
 *
 * **Constraints:**
 *
 * * `k == lists.length`
 * * `0 <= k <= 10^4`
 * * `0 <= lists[i].length <= 500`
 * * `-10^4 <= lists[i][j] <= 10^4`
 * * `lists[i]` is sorted in **ascending order**.
 * * The sum of `lists[i].length` won't exceed `10^4`.
 *
 */

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linked_list::to_list;

    #[test]
    fn test_23() {
        for case in vec![(
            vec![
                to_list(vec![1, 4, 5]),
                to_list(vec![1, 3, 4]),
                to_list(vec![2, 6]),
            ],
            to_list(vec![1, 1, 2, 3, 4, 4, 5, 6]),
        )] {
            let (lists, wanted) = case;
            assert_eq!(Solution::merge_k_lists(lists), wanted);
        }
    }
}

struct Solution {}

// https://leetcode.com/problems/merge-k-sorted-lists

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
use crate::linked_list::ListNode;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // descending
        other.val.partial_cmp(&self.val)
    }
}

impl Ord for ListNode {
    fn cmp(&self, _: &Self) -> Ordering {
        unimplemented!()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Node(Option<Box<ListNode>>);

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap: BinaryHeap<Node> = lists.into_iter().map(|v| Node(v)).collect();
        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let mut current = &mut dummy_head;
        while !heap.is_empty() {
            let mut min_node = heap.pop().unwrap().0;
            if min_node.is_none() {
                continue;
            }

            let next = min_node.as_mut().unwrap().next.take();
            if next.is_some() {
                heap.push(Node(next));
            }

            current.as_mut().unwrap().next = min_node;
            current = &mut current.as_mut().unwrap().next
        }
        dummy_head.unwrap().next
    }
}
