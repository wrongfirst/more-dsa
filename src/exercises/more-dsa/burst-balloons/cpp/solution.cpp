class Solution {
public:
    int maxCoins(std::vector<int>& nums) {
        std::unordered_map<long, int> cache; // Using long for key to combine two integers uniquely
        nums.insert(nums.begin(), 1);
        nums.push_back(1);
        
        auto getKey = [](int left, int right) -> long {
            return static_cast<long>(left) << 32 | static_cast<long>(right);
        };
        
        for (int offset = 2; offset < nums.size(); ++offset) {
            for (int left = 0; left < nums.size() - offset; ++left) {
                int right = left + offset;
                for (int pivot = left + 1; pivot < right; ++pivot) {
                    int coins = nums[left] * nums[pivot] * nums[right];
                    coins += cache[getKey(left, pivot)] + cache[getKey(pivot, right)];
                    long key = getKey(left, right);
                    cache[key] = std::max(coins, cache[key]);
                }
            }
        }
        return cache[getKey(0, nums.size() - 1)];
    }
};
