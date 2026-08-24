use std::collections::HashMap;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut dp: HashMap<(usize, bool), i32> = HashMap::new();
        Self::dfs(&prices, 0, true, &mut dp)
    }

    fn dfs(prices: &[i32], i: usize, buying: bool, dp: &mut HashMap<(usize, bool), i32>) -> i32 {
        if i >= prices.len() {
            return 0;
        }
        if let Some(&result) = dp.get(&(i, buying)) {
            return result;
        }

        let cooldown = Self::dfs(prices, i + 1, buying, dp);
        let result = if buying {
            let buy = Self::dfs(prices, i + 1, false, dp) - prices[i];
            buy.max(cooldown)
        } else {
            let sell = Self::dfs(prices, i + 2, true, dp) + prices[i];
            sell.max(cooldown)
        };

        dp.insert((i, buying), result);
        result
    }
}
