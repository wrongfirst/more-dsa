// Definition for a Node.
// #[derive(Clone, Debug, PartialEq, Eq)]
// pub struct Node {
//     pub val: i32,
//     pub next: Option<Rc<RefCell<Node>>>,
//     pub random: Option<Rc<RefCell<Node>>>,
// }
//
// impl Node {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         Node {
//             val,
//             next: None,
//             random: None,
//         }
//     }
// }

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

impl Solution {
    pub fn copy_random_list(head: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        let mut old_to_copy: HashMap<*const RefCell<Node>, Rc<RefCell<Node>>> = HashMap::new();

        // First pass: create all nodes
        let mut cur = head.clone();
        while let Some(node) = cur {
            let node_ref = node.borrow();
            let copy = Rc::new(RefCell::new(Node::new(node_ref.val)));
            old_to_copy.insert(Rc::as_ptr(&node), copy);
            cur = node_ref.next.clone();
        }

        // Second pass: set next and random pointers
        cur = head.clone();
        while let Some(node) = cur {
            let node_ref = node.borrow();
            let copy = old_to_copy.get(&Rc::as_ptr(&node)).unwrap().clone();

            if let Some(next_node) = &node_ref.next {
                let next_copy = old_to_copy.get(&Rc::as_ptr(next_node)).unwrap().clone();
                copy.borrow_mut().next = Some(next_copy);
            }

            if let Some(random_node) = &node_ref.random {
                let random_copy = old_to_copy.get(&Rc::as_ptr(random_node)).unwrap().clone();
                copy.borrow_mut().random = Some(random_copy);
            }

            cur = node_ref.next.clone();
        }

        head.as_ref().map(|node| old_to_copy.get(&Rc::as_ptr(node)).unwrap().clone())
    }
}
