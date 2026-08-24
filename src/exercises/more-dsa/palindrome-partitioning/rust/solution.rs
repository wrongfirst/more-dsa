impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut res: Vec<Vec<String>> = Vec::new();
        let mut part: Vec<String> = Vec::new();
        let chars: Vec<char> = s.chars().collect();
        Self::dfs(&chars, 0, &mut part, &mut res);
        res
    }

    fn dfs(chars: &[char], i: usize, part: &mut Vec<String>, res: &mut Vec<Vec<String>>) {
        if i >= chars.len() {
            res.push(part.clone());
            return;
        }
        for j in i..chars.len() {
            if Self::is_palindrome(chars, i, j) {
                part.push(chars[i..=j].iter().collect());
                Self::dfs(chars, j + 1, part, res);
                part.pop();
            }
        }
    }

    fn is_palindrome(chars: &[char], mut l: usize, mut r: usize) -> bool {
        while l < r {
            if chars[l] != chars[r] {
                return false;
            }
            l += 1;
            r -= 1;
        }
        true
    }
}
