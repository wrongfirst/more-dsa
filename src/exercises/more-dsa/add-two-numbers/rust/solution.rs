// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }
//
// impl ListNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut cur = &mut dummy;

        let mut l1 = l1;
        let mut l2 = l2;
        let mut carry = 0;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let v1 = l1.as_ref().map_or(0, |node| node.val);
            let v2 = l2.as_ref().map_or(0, |node| node.val);

            let mut val = v1 + v2 + carry;
            carry = val / 10;
            val = val % 10;

            cur.next = Some(Box::new(ListNode::new(val)));
            cur = cur.next.as_mut().unwrap();

            l1 = l1.and_then(|node| node.next);
            l2 = l2.and_then(|node| node.next);
        }

        dummy.next
    }
}
