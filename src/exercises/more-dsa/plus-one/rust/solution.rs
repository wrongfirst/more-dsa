impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits: Vec<i32> = digits.into_iter().rev().collect();
        let mut carry = 1;
        let mut i = 0;

        while carry != 0 {
            if i < digits.len() {
                if digits[i] == 9 {
                    digits[i] = 0;
                } else {
                    digits[i] += 1;
                    carry = 0;
                }
            } else {
                digits.push(carry);
                carry = 0;
            }
            i += 1;
        }

        digits.into_iter().rev().collect()
    }
}
