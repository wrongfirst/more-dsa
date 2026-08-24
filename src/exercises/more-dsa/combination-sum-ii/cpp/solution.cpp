class Solution {
public:
    vector<vector<int>> combinationSum2(vector<int>& candidates, int target) {
        vector<vector<int>> res;
        sort(candidates.begin(), candidates.end());
        backtrack({}, 0, target, candidates, res);
        return res;
    }

private:
    void backtrack(vector<int> cur, int pos, int target, vector<int>& candidates, vector<vector<int>>& res) {
        if (target == 0) {
            res.push_back(cur);
            return;
        }
        if (target < 0) {
            return;
        }

        int prev = -1;
        for (int i = pos; i < candidates.size(); i++) {
            if (candidates[i] == prev) {
                continue;
            }
            cur.push_back(candidates[i]);
            backtrack(cur, i + 1, target - candidates[i], candidates, res);
            cur.pop_back();
            prev = candidates[i];
        }
    }
};
