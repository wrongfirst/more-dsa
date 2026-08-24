use std::collections::HashMap;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return Vec::new();
        }

        let digit_to_char: HashMap<char, &str> = [
            ('2', "abc"),
            ('3', "def"),
            ('4', "ghi"),
            ('5', "jkl"),
            ('6', "mno"),
            ('7', "pqrs"),
            ('8', "tuv"),
            ('9', "wxyz"),
        ].iter().cloned().collect();

        let mut res = Vec::new();
        Self::backtrack(0, &digits, String::new(), &mut res, &digit_to_char);
        res
    }

    fn backtrack(
        i: usize,
        digits: &str,
        cur_str: String,
        res: &mut Vec<String>,
        digit_to_char: &HashMap<char, &str>,
    ) {
        if cur_str.len() == digits.len() {
            res.push(cur_str);
            return;
        }

        let digit = digits.chars().nth(i).unwrap();
        for c in digit_to_char.get(&digit).unwrap().chars() {
            let mut new_str = cur_str.clone();
            new_str.push(c);
            Self::backtrack(i + 1, digits, new_str, res, digit_to_char);
        }
    }
}
