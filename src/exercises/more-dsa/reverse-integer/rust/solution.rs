impl Solution {
    pub fn reverse(x: i32) -> i32 {
        const MIN: i32 = i32::MIN; // -2^31
        const MAX: i32 = i32::MAX; // 2^31 - 1

        let mut x = x;
        let mut res: i32 = 0;

        while x != 0 {
            let digit = x % 10;
            x /= 10;

            if res > MAX / 10 || (res == MAX / 10 && digit > MAX % 10) {
                return 0;
            }
            if res < MIN / 10 || (res == MIN / 10 && digit < MIN % 10) {
                return 0;
            }
            res = res * 10 + digit;
        }

        res
    }
}
