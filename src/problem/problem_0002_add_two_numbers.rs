/**
 *
 * https://leetcode.com/problems/add-two-numbers
 *
 * 2. Add Two Numbers
 *
 * You are given two **non-empty** linked lists representing two non-negative
 *integers. The digits are stored in **reverse order**, and each of their nodes
 *contains a single digit. Add the two numbers and return the sum as a linked
 *list.
 *
 *You may assume the two numbers do not contain any leading zero, except the
 *number 0 itself.
 *
 *
 ***Example 1:**
 *
 *[]
 ***Input:** l1 = [2,4,3], l2 = [5,6,4]
 ***Output:** [7,0,8]
 ***Explanation:** 342 + 465 = 807.
 *
 ***Example 2:**
 *
 ***Input:** l1 = [0], l2 = [0]
 ***Output:** [0]
 *
 ***Example 3:**
 *
 ***Input:** l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
 ***Output:** [8,9,9,9,0,0,0,1]
 *
 *
 ***Constraints:**
 *
 ** The number of nodes in each linked list is in the range `[1, 100]`.
 ** `0 <= Node.val <= 9`
 ** It is guaranteed that the list represents a number that does not have leading
 *  zeros.
 *
 */

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linked_list::to_list;

    #[test]
    fn test_2() {
        for case in vec![
            (vec![2, 4, 3], vec![5, 6, 4], vec![7, 0, 8]),
            (vec![0], vec![0], vec![0]),
            (
                vec![9, 9, 9, 9, 9, 9, 9],
                vec![9, 9, 9, 9],
                vec![8, 9, 9, 9, 0, 0, 0, 1],
            ),
        ] {
            let (l1, l2, wanted) = case;
            assert_eq!(
                Solution::add_two_numbers(to_list(l1), to_list(l2)),
                to_list(wanted)
            );
        }
    }
}

struct Solution {}

use crate::linked_list::ListNode;

// https://leetcode.com/problems/add-two-numbers

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

struct ListIter<'a> {
    current: &'a Option<Box<ListNode>>,
}

impl Iterator for ListIter<'_> {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            None => None,
            Some(node) => {
                let v = Some(node.val);
                self.current = &node.next;
                return v;
            }
        }
    }
}

fn iter(list: &Option<Box<ListNode>>) -> ListIter {
    ListIter { current: list }
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = iter(&l1).fuse();
        let mut l2 = iter(&l2).fuse();

        let mut carry_digit = 0;
        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut dummy_head;

        loop {
            let a1;
            let a2;

            match (l1.next(), l2.next()) {
                (None, None) => {
                    if carry_digit > 0 {
                        tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(carry_digit)));
                    }
                    return dummy_head.unwrap().next;
                }
                (Some(i), None) => {
                    a1 = i;
                    a2 = 0;
                }
                (None, Some(j)) => {
                    a1 = 0;
                    a2 = j;
                }
                (Some(i), Some(j)) => {
                    a1 = i;
                    a2 = j;
                }
            }

            let mut current_digit = a1 + a2 + carry_digit;
            if current_digit > 9 {
                carry_digit = current_digit / 10;
                current_digit = current_digit % 10;
            } else {
                carry_digit = 0;
            }

            tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(current_digit)));
            tail = &mut tail.as_mut().unwrap().next;
        }
    }
}
