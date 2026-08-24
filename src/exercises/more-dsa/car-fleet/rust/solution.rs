impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let n = position.len();
        let mut pairs: Vec<(i32, i32)> = position.iter().zip(speed.iter())
            .map(|(&p, &s)| (p, s))
            .collect();
        pairs.sort_by(|a, b| b.0.cmp(&a.0));

        let mut fleet_count = 0;
        let mut time_to_reach: Vec<f64> = vec![0.0; n];

        for i in 0..n {
            time_to_reach[i] = (target - pairs[i].0) as f64 / pairs[i].1 as f64;
            if i >= 1 && time_to_reach[i] <= time_to_reach[i - 1] {
                time_to_reach[i] = time_to_reach[i - 1];
            } else {
                fleet_count += 1;
            }
        }
        fleet_count
    }
}
