class Solution {
public:
    vector<vector<int>> permute(vector<int>& nums) {
        vector<vector<int>> res;

        if (nums.size() == 1) {
            vector<int> singleNum;
            singleNum.push_back(nums[0]);
            res.push_back(singleNum);
            return res;
        }

        for (size_t i = 0; i < nums.size(); i++) {
            int n = nums[i];
            vector<int> remainingNums(nums.size() - 1);
            size_t idx = 0;
            for (size_t j = 0; j < nums.size(); j++) {
                if (j != i) {
                    remainingNums[idx++] = nums[j];
                }
            }
            vector<vector<int>> perms = permute(remainingNums);

            for (const auto& perm : perms) {
                vector<int> newPerm = perm;
                newPerm.push_back(n);
                res.push_back(newPerm);
            }
        }
        return res;
    }
};
