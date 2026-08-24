impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort();
        let mut res: Vec<Vec<i32>> = Vec::new();
        Self::backtrack(&candidates, target, 0, &mut Vec::new(), &mut res);
        res
    }

    fn backtrack(
        candidates: &[i32],
        target: i32,
        pos: usize,
        cur: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
    ) {
        if target == 0 {
            res.push(cur.clone());
            return;
        }
        if target < 0 {
            return;
        }

        let mut prev = -1;
        for i in pos..candidates.len() {
            if candidates[i] == prev {
                continue;
            }
            cur.push(candidates[i]);
            Self::backtrack(candidates, target - candidates[i], i + 1, cur, res);
            cur.pop();
            prev = candidates[i];
        }
    }
}
