use std::collections::HashMap;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();
        let mut cache: HashMap<(usize, usize), i32> = HashMap::new();

        // Base cases
        for i in 0..=s.len() {
            cache.insert((i, t.len()), 1);
        }
        for j in 0..t.len() {
            cache.insert((s.len(), j), 0);
        }

        // Fill the cache bottom-up
        for i in (0..s.len()).rev() {
            for j in (0..t.len()).rev() {
                if s[i] == t[j] {
                    let val1 = *cache.get(&(i + 1, j + 1)).unwrap();
                    let val2 = *cache.get(&(i + 1, j)).unwrap();
                    cache.insert((i, j), val1 + val2);
                } else {
                    let val = *cache.get(&(i + 1, j)).unwrap();
                    cache.insert((i, j), val);
                }
            }
        }

        *cache.get(&(0, 0)).unwrap()
    }
}
