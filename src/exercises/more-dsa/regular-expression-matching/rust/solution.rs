impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();
        let n = s.len();
        let m = p.len();

        let mut cache = vec![vec![false; m + 1]; n + 1];
        cache[n][m] = true;

        for i in (0..=n).rev() {
            for j in (0..m).rev() {
                let is_match = i < n && (s[i] == p[j] || p[j] == '.');

                if j + 1 < m && p[j + 1] == '*' {
                    cache[i][j] = cache[i][j + 2];
                    if is_match {
                        cache[i][j] = cache[i + 1][j] || cache[i][j];
                    }
                } else if is_match {
                    cache[i][j] = cache[i + 1][j + 1];
                }
            }
        }

        cache[0][0]
    }
}
