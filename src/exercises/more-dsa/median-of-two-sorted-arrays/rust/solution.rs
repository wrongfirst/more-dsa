impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (a, b) = if nums1.len() <= nums2.len() {
            (&nums1, &nums2)
        } else {
            (&nums2, &nums1)
        };

        let total = a.len() + b.len();
        let half = total / 2;

        let mut l: i32 = 0;
        let mut r: i32 = a.len() as i32 - 1;

        loop {
            // Floor division: equivalent to Python's (l + r) // 2
            let i = floor_div(l + r, 2);
            let j = half as i32 - i - 2;

            let a_left = if i >= 0 {
                a[i as usize] as f64
            } else {
                f64::NEG_INFINITY
            };
            let a_right = if (i + 1) < a.len() as i32 {
                a[(i + 1) as usize] as f64
            } else {
                f64::INFINITY
            };
            let b_left = if j >= 0 {
                b[j as usize] as f64
            } else {
                f64::NEG_INFINITY
            };
            let b_right = if (j + 1) < b.len() as i32 {
                b[(j + 1) as usize] as f64
            } else {
                f64::INFINITY
            };

            if a_left <= b_right && b_left <= a_right {
                if total % 2 == 1 {
                    return a_right.min(b_right);
                }
                return (a_left.max(b_left) + a_right.min(b_right)) / 2.0;
            } else if a_left > b_right {
                r = i - 1;
            } else {
                l = i + 1;
            }
        }
    }
}

fn floor_div(a: i32, b: i32) -> i32 {
    let d = a / b;
    let r = a % b;
    if (r != 0) && ((r ^ b) < 0) {
        d - 1
    } else {
        d
    }
}
