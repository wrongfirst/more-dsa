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

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::dfs(&root).0
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> (bool, i32) {
        match root {
            None => (true, 0),
            Some(node) => {
                let node_ref = node.borrow();
                let (left_balanced, left_height) = Self::dfs(&node_ref.left);
                let (right_balanced, right_height) = Self::dfs(&node_ref.right);

                let balanced = left_balanced && right_balanced && (left_height - right_height).abs() <= 1;
                let height = 1 + left_height.max(right_height);

                (balanced, height)
            }
        }
    }
}
