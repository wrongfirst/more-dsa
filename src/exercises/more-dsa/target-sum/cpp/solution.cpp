class Solution {
public:
    int findTargetSumWays(vector<int>& nums, int target) {
        unordered_map<long, int> dp; // Changed to long for key
        return backtrack(0, 0, nums, target, dp);
    }

    int backtrack(int i, int total, vector<int>& nums, int target, unordered_map<long, int>& dp) {
        if (i == nums.size()) {
            return total == target ? 1 : 0;
        }
        long key = static_cast<long>(i) << 32 | (total & 0xffffffffL); // Combine i and total into a single key
        if (dp.find(key) != dp.end()) {
            return dp[key];
        }

        int ways = backtrack(i + 1, total + nums[i], nums, target, dp) 
                + backtrack(i + 1, total - nums[i], nums, target, dp);
        dp[key] = ways;
        return ways;
    }
};
