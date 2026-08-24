impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let n = n as usize;
        let src = src as usize;
        let dst = dst as usize;

        let mut prices = vec![i32::MAX; n];
        prices[src] = 0;

        for _ in 0..=k {
            let tmp_prices = prices.clone();
            let mut updated = prices.clone();

            for flight in &flights {
                let s = flight[0] as usize;
                let d = flight[1] as usize;
                let p = flight[2];

                if tmp_prices[s] == i32::MAX {
                    continue;
                }

                if tmp_prices[s] + p < updated[d] {
                    updated[d] = tmp_prices[s] + p;
                }
            }

            prices = updated;
        }

        if prices[dst] == i32::MAX {
            -1
        } else {
            prices[dst]
        }
    }
}
