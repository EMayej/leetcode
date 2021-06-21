/**
 *
 * https://leetcode.com/problems/swap-nodes-in-pairs
 *
 * 24. Swap Nodes in Pairs
 *
 * Given a linked list, swap every two adjacent nodes and return its head. You must
 * solve the problem without modifying the values in the list's nodes (i.e., only
 * nodes themselves may be changed.)
 *
 *
 * **Example 1:**
 *
 * []
 * **Input:** head = [1,2,3,4]
 * **Output:** [2,1,4,3]
 *
 * **Example 2:**
 *
 * **Input:** head = []
 * **Output:** []
 *
 * **Example 3:**
 *
 * **Input:** head = [1]
 * **Output:** [1]
 *
 *
 * **Constraints:**
 *
 * * The number of nodes in the list is in the range `[0, 100]`.
 * * `0 <= Node.val <= 100`
 *
 */

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linked_list::to_list;

    #[test]
    fn test_24() {
        for case in vec![
            (to_list(vec![]), to_list(vec![])),
            (to_list(vec![1]), to_list(vec![1])),
            (to_list(vec![1, 2]), to_list(vec![2, 1])),
            (to_list(vec![1, 2, 3]), to_list(vec![2, 1, 3])),
            (to_list(vec![1, 2, 3, 4]), to_list(vec![2, 1, 4, 3])),
        ] {
            let (head, wanted) = case;
            assert_eq!(Solution::swap_pairs(head), wanted);
        }
    }
}

struct Solution {}

// https://leetcode.com/problems/swap-nodes-in-pairs

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

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));
        let mut head = dummy_head.as_mut();
        loop {
            let mut left = head.as_mut().unwrap().next.take();
            if left.is_none() {
                break;
            }
            let mut right = left.as_mut().unwrap().next.take();
            // handle the un-paired one, e.g. [1, 2, 3] -> [2, 1, 3], 3 is un-paired
            if right.is_none() {
                head.as_mut().unwrap().next = left;
                break;
            }
            let next = right.as_mut().unwrap().next.take();
            // BEFORE: head -> left -> right -> next
            // AFTER: head -> right -> left -> next
            left.as_mut().unwrap().next = next;
            right.as_mut().unwrap().next = left;
            head.as_mut().unwrap().next = right;
            head = head.unwrap().next.as_mut().unwrap().next.as_mut();
        }
        dummy_head.unwrap().next
    }
}
