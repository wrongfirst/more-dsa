impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut l: i64 = 1;
        let mut r: i64 = *piles.iter().max().unwrap() as i64;
        let mut res = r;
        let h = h as i64;

        while l <= r {
            let k = (l + r) / 2;

            let mut total_time: i64 = 0;
            for &p in &piles {
                total_time += ((p as i64) + k - 1) / k; // ceiling division
            }

            if total_time <= h {
                res = k;
                r = k - 1;
            } else {
                l = k + 1;
            }
        }

        res as i32
    }
}
