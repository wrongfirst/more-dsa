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
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let k = k as usize;

        // Count nodes
        let mut count = 0;
        let mut cur = &head;
        while let Some(node) = cur {
            count += 1;
            cur = &node.next;
        }

        if count < k {
            return head;
        }

        // Recursive approach: reverse first k nodes, then recursively handle rest
        let mut head = head;
        let mut prev: Option<Box<ListNode>> = None;
        let mut curr = head;

        for _ in 0..k {
            let mut node = curr.unwrap();
            curr = node.next.take();
            node.next = prev;
            prev = Some(node);
        }

        // prev is now the new head of this group
        // The original head is now the tail of this reversed group
        // Find the tail and connect it to the result of recursive call
        let mut tail = &mut prev;
        for _ in 0..k - 1 {
            tail = &mut tail.as_mut().unwrap().next;
        }

        // Recursively reverse the remaining groups
        tail.as_mut().unwrap().next = Self::reverse_k_group(curr, k as i32);

        prev
    }
}
