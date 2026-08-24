impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }

        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();
        let s3: Vec<char> = s3.chars().collect();

        let m = s1.len();
        let n = s2.len();

        let mut dp = vec![vec![false; n + 1]; m + 1];
        dp[m][n] = true;

        for i in (0..=m).rev() {
            for j in (0..=n).rev() {
                if i < m && s1[i] == s3[i + j] && dp[i + 1][j] {
                    dp[i][j] = true;
                }
                if j < n && s2[j] == s3[i + j] && dp[i][j + 1] {
                    dp[i][j] = true;
                }
            }
        }

        dp[0][0]
    }
}
