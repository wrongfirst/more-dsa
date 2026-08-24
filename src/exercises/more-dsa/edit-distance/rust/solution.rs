impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1: Vec<char> = word1.chars().collect();
        let word2: Vec<char> = word2.chars().collect();
        let m = word1.len();
        let n = word2.len();

        let mut dp = vec![vec![i32::MAX; n + 1]; m + 1];

        for j in 0..=n {
            dp[m][j] = (n - j) as i32;
        }
        for i in 0..=m {
            dp[i][n] = (m - i) as i32;
        }

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                if word1[i] == word2[j] {
                    dp[i][j] = dp[i + 1][j + 1];
                } else {
                    dp[i][j] = 1 + dp[i + 1][j].min(dp[i][j + 1]).min(dp[i + 1][j + 1]);
                }
            }
        }

        dp[0][0]
    }
}
