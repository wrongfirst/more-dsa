impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let n = temperatures.len();
        let mut res = vec![0; n];
        let mut stack: Vec<(i32, usize)> = Vec::new(); // (temp, index)

        for (i, &t) in temperatures.iter().enumerate() {
            while !stack.is_empty() && t > stack.last().unwrap().0 {
                let (_, stack_idx) = stack.pop().unwrap();
                res[stack_idx] = (i - stack_idx) as i32;
            }
            stack.push((t, i));
        }

        res
    }
}
