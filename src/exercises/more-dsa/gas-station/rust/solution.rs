impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let total_gas: i32 = gas.iter().sum();
        let total_cost: i32 = cost.iter().sum();

        if total_gas < total_cost {
            return -1;
        }

        let mut total = 0;
        let mut res = 0;

        for i in 0..gas.len() {
            total += gas[i] - cost[i];

            if total < 0 {
                total = 0;
                res = (i + 1) as i32;
            }
        }

        res
    }
}
