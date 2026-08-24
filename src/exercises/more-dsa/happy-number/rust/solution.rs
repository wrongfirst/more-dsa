impl Solution {
    pub fn is_happy(n: i32) -> bool {
        fn sum_square_digits(mut n: i32) -> i32 {
            let mut output = 0;
            while n > 0 {
                let digit = n % 10;
                output += digit * digit;
                n /= 10;
            }
            output
        }

        let mut slow = n;
        let mut fast = sum_square_digits(n);

        while slow != fast {
            fast = sum_square_digits(fast);
            fast = sum_square_digits(fast);
            slow = sum_square_digits(slow);
        }

        fast == 1
    }
}
