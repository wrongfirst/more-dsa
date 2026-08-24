use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Reverse;

struct Twitter {
    count: i32,
    tweet_map: HashMap<i32, Vec<(i32, i32)>>,
    follow_map: HashMap<i32, HashSet<i32>>,
}

impl Twitter {
    pub fn new() -> Self {
        Twitter {
            count: 0,
            tweet_map: HashMap::new(),
            follow_map: HashMap::new(),
        }
    }

    pub fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.tweet_map
            .entry(user_id)
            .or_insert_with(Vec::new)
            .push((self.count, tweet_id));
        self.count -= 1;
    }

    pub fn get_news_feed(&mut self, user_id: i32) -> Vec<i32> {
        let mut res = Vec::new();
        let mut min_heap: BinaryHeap<Reverse<(i32, i32, i32, usize)>> = BinaryHeap::new();

        self.follow_map
            .entry(user_id)
            .or_insert_with(HashSet::new)
            .insert(user_id);

        if let Some(followees) = self.follow_map.get(&user_id) {
            for &followee_id in followees.iter() {
                if let Some(tweets) = self.tweet_map.get(&followee_id) {
                    if !tweets.is_empty() {
                        let idx = tweets.len() - 1;
                        let (count, tweet_id) = tweets[idx];
                        min_heap.push(Reverse((count, tweet_id, followee_id, idx)));
                    }
                }
            }
        }

        while !min_heap.is_empty() && res.len() < 10 {
            let Reverse((_, tweet_id, followee_id, idx)) = min_heap.pop().unwrap();
            res.push(tweet_id);

            if idx > 0 {
                if let Some(tweets) = self.tweet_map.get(&followee_id) {
                    let new_idx = idx - 1;
                    let (count, tweet_id) = tweets[new_idx];
                    min_heap.push(Reverse((count, tweet_id, followee_id, new_idx)));
                }
            }
        }

        res
    }

    pub fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.follow_map
            .entry(follower_id)
            .or_insert_with(HashSet::new)
            .insert(followee_id);
    }

    pub fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(followees) = self.follow_map.get_mut(&follower_id) {
            followees.remove(&followee_id);
        }
    }
}
