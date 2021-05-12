// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn to_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode::new(0)));

    let mut head = &mut dummy;
    for n in nums {
        let node = ListNode::new(n);
        head.as_mut().unwrap().next = Some(Box::new(node));
        head = &mut head.as_mut().unwrap().next;
    }

    dummy.unwrap().next
}
