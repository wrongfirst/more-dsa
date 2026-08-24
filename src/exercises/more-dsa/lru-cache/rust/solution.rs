use std::collections::HashMap;

struct Node {
    key: i32,
    val: i32,
    prev: *mut Node,
    next: *mut Node,
}

impl Node {
    fn new(key: i32, val: i32) -> Self {
        Node {
            key,
            val,
            prev: std::ptr::null_mut(),
            next: std::ptr::null_mut(),
        }
    }
}

struct LRUCache {
    capacity: usize,
    cache: HashMap<i32, *mut Node>,
    left: *mut Node,
    right: *mut Node,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        let left = Box::into_raw(Box::new(Node::new(0, 0)));
        let right = Box::into_raw(Box::new(Node::new(0, 0)));

        unsafe {
            (*left).next = right;
            (*right).prev = left;
        }

        LRUCache {
            capacity: capacity as usize,
            cache: HashMap::new(),
            left,
            right,
        }
    }

    fn remove(&mut self, node: *mut Node) {
        unsafe {
            let prev = (*node).prev;
            let next = (*node).next;
            (*prev).next = next;
            (*next).prev = prev;
        }
    }

    fn insert(&mut self, node: *mut Node) {
        unsafe {
            let prev = (*self.right).prev;
            let next = self.right;
            (*prev).next = node;
            (*next).prev = node;
            (*node).prev = prev;
            (*node).next = next;
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(&node) = self.cache.get(&key) {
            self.remove(node);
            self.insert(node);
            unsafe { (*node).val }
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(&node) = self.cache.get(&key) {
            self.remove(node);
            unsafe {
                (*node).val = value;
            }
            self.insert(node);
        } else {
            let node = Box::into_raw(Box::new(Node::new(key, value)));
            self.cache.insert(key, node);
            self.insert(node);

            if self.cache.len() > self.capacity {
                unsafe {
                    let lru = (*self.left).next;
                    self.remove(lru);
                    self.cache.remove(&(*lru).key);
                    drop(Box::from_raw(lru));
                }
            }
        }
    }
}

impl Drop for LRUCache {
    fn drop(&mut self) {
        unsafe {
            for (_, &node) in self.cache.iter() {
                drop(Box::from_raw(node));
            }
            drop(Box::from_raw(self.left));
            drop(Box::from_raw(self.right));
        }
    }
}
