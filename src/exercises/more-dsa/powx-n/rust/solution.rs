impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        fn helper(x: f64, n: i64) -> f64 {
            if x == 0.0 {
                return 0.0;
            }
            if n == 0 {
                return 1.0;
            }

            let res = helper(x * x, n / 2);
            if n % 2 == 1 {
                x * res
            } else {
                res
            }
        }

        let n_long = n as i64;
        let res = helper(x, n_long.abs());
        if n >= 0 {
            res
        } else {
            1.0 / res
        }
    }
}
