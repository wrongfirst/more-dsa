class Solution {
public:
    int maxProfit(vector<int>& prices) {
        unordered_map<long, int> dp; // Use long for the combined key
        return dfs(0, true, prices, dp);
    }

    int dfs(int i, bool buying, vector<int>& prices, unordered_map<long, int>& dp) {
        if (i >= prices.size()) {
            return 0;
        }
        
        // Combine 'i' and 'buying' into a single long key
        long key = static_cast<long>(i) << 1 | static_cast<long>(buying);
        if (dp.find(key) != dp.end()) {
            return dp[key];
        }

        int cooldown = dfs(i + 1, buying, prices, dp);
        if (buying) {
            int buy = dfs(i + 1, !buying, prices, dp) - prices[i];
            dp[key] = max(buy, cooldown);
        } else {
            int sell = dfs(i + 2, !buying, prices, dp) + prices[i];
            dp[key] = max(sell, cooldown);
        }
        return dp[key];
    }
};
