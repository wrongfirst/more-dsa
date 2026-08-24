impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut left_min = 0i32;
        let mut left_max = 0i32;

        for c in s.chars() {
            if c == '(' {
                left_min += 1;
                left_max += 1;
            } else if c == ')' {
                left_min -= 1;
                left_max -= 1;
            } else {
                left_min -= 1;
                left_max += 1;
            }
            if left_max < 0 {
                return false;
            }
            if left_min < 0 {
                left_min = 0;
            }
        }
        left_min == 0
    }
}
