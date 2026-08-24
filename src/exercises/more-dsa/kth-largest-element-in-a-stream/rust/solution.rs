use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    k: usize,
    min_heap: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut kth = KthLargest {
            k: k as usize,
            min_heap: BinaryHeap::new(),
        };

        for num in nums {
            kth.min_heap.push(Reverse(num));
        }

        while kth.min_heap.len() > kth.k {
            kth.min_heap.pop();
        }

        kth
    }

    pub fn add(&mut self, val: i32) -> i32 {
        self.min_heap.push(Reverse(val));

        if self.min_heap.len() > self.k {
            self.min_heap.pop();
        }

        self.min_heap.peek().unwrap().0
    }
}
