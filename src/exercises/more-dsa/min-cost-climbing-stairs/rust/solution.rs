impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut cost = cost;
        let n = cost.len();

        for i in (0..n.saturating_sub(2)).rev() {
            cost[i] += cost[i + 1].min(cost[i + 2]);
        }

        cost[0].min(cost[1])
    }
}
