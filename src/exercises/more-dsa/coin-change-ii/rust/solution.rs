impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let amount = amount as usize;
        let mut dp = vec![0; amount + 1];
        dp[0] = 1;

        for i in (0..coins.len()).rev() {
            let mut next_dp = vec![0; amount + 1];
            next_dp[0] = 1;

            for a in 1..=amount {
                next_dp[a] = dp[a];
                let coin = coins[i] as usize;
                if a >= coin {
                    next_dp[a] += next_dp[a - coin];
                }
            }
            dp = next_dp;
        }

        dp[amount]
    }
}
