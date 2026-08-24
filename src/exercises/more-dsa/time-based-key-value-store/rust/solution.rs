use std::collections::HashMap;

struct TimeMap {
    key_store: HashMap<String, Vec<(i32, String)>>,
}

impl TimeMap {
    fn new() -> Self {
        TimeMap {
            key_store: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.key_store
            .entry(key)
            .or_insert_with(Vec::new)
            .push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(values) = self.key_store.get(&key) {
            let mut left = 0;
            let mut right = values.len() as i32 - 1;
            let mut result = String::new();

            while left <= right {
                let mid = left + (right - left) / 2;
                if values[mid as usize].0 <= timestamp {
                    result = values[mid as usize].1.clone();
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }

            result
        } else {
            String::new()
        }
    }
}
