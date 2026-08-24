// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut q: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        q.push_back(root);

        while !q.is_empty() {
            let mut right_side: Option<i32> = None;
            let q_len = q.len();

            for _ in 0..q_len {
                if let Some(node_opt) = q.pop_front() {
                    if let Some(node) = node_opt {
                        let node_ref = node.borrow();
                        right_side = Some(node_ref.val);
                        q.push_back(node_ref.left.clone());
                        q.push_back(node_ref.right.clone());
                    }
                }
            }

            if let Some(val) = right_side {
                res.push(val);
            }
        }

        res
    }
}
