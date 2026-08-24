use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        if !word_list.contains(&end_word) {
            return 0;
        }

        let mut nei: HashMap<String, Vec<String>> = HashMap::new();
        let mut all_words = word_list.clone();
        all_words.push(begin_word.clone());

        for word in &all_words {
            for j in 0..word.len() {
                let pattern = format!("{}*{}", &word[..j], &word[j + 1..]);
                nei.entry(pattern).or_default().push(word.clone());
            }
        }

        let mut visit: HashSet<String> = HashSet::new();
        let mut queue: VecDeque<String> = VecDeque::new();
        queue.push_back(begin_word.clone());
        let mut res = 1;

        while !queue.is_empty() {
            let size = queue.len();
            for _ in 0..size {
                let word = queue.pop_front().unwrap();
                if word == end_word {
                    return res;
                }
                for j in 0..word.len() {
                    let pattern = format!("{}*{}", &word[..j], &word[j + 1..]);
                    if let Some(neighbors) = nei.get(&pattern) {
                        for nei_word in neighbors {
                            if !visit.contains(nei_word) {
                                visit.insert(nei_word.clone());
                                queue.push_back(nei_word.clone());
                            }
                        }
                    }
                }
            }
            res += 1;
        }

        0
    }
}
