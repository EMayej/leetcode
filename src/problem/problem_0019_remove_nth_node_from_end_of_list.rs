/**
 *
 * https://leetcode.com/problems/remove-nth-node-from-end-of-list
 *
 * 19. Remove Nth Node From End of List
 *
 * Given the `head` of a linked list, remove the `nth` node from the end of the
 * list and return its head.
 *
 *
 * **Example 1:**
 *
 * []
 * **Input:** head = [1,2,3,4,5], n = 2
 * **Output:** [1,2,3,5]
 *
 * **Example 2:**
 *
 * **Input:** head = [1], n = 1
 * **Output:** []
 *
 * **Example 3:**
 *
 * **Input:** head = [1,2], n = 1
 * **Output:** [1]
 *
 *
 * **Constraints:**
 *
 * * The number of nodes in the list is `sz`.
 * * `1 <= sz <= 30`
 * * `0 <= Node.val <= 100`
 * * `1 <= n <= sz`
 *
 *
 * **Follow up:** Could you do this in one pass?
 *
 */

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linked_list::to_list;

    #[test]
    fn test_19() {
        for case in vec![(to_list(vec![1, 2, 3, 4, 5]), 2, to_list(vec![1, 2, 3, 5]))] {
            let (head, n, wanted) = case;
            assert_eq!(Solution::remove_nth_from_end(head, n), wanted);
        }
    }
}

struct Solution {}

// https://leetcode.com/problems/remove-nth-node-from-end-of-list

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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));
        let mut len = 0;
        {
            let mut p = dummy_head.as_ref();
            while p.unwrap().next.is_some() {
                len += 1;
                p = p.unwrap().next.as_ref();
            }
        }
        let index = len - n;
        {
            let mut p = dummy_head.as_mut();
            for _ in 0..index {
                p = p.unwrap().next.as_mut();
            }
            let next = p.as_mut().unwrap().next.as_mut().unwrap().next.take();
            p.as_mut().unwrap().next = next;
        }
        dummy_head.unwrap().next
    }
}
