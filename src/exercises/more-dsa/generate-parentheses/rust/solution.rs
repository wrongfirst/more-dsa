impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res = Vec::new();
        let mut stack = String::new();
        Self::backtrack(&mut res, &mut stack, 0, 0, n);
        res
    }

    fn backtrack(res: &mut Vec<String>, stack: &mut String, open_n: i32, closed_n: i32, n: i32) {
        if open_n == n && closed_n == n {
            res.push(stack.clone());
            return;
        }

        if open_n < n {
            stack.push('(');
            Self::backtrack(res, stack, open_n + 1, closed_n, n);
            stack.pop();
        }

        if closed_n < open_n {
            stack.push(')');
            Self::backtrack(res, stack, open_n, closed_n + 1, n);
            stack.pop();
        }
    }
}
