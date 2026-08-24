// Quick Select
// Time complexity: O(n) in average, O(n^2) in worst case
class Solution {
public:
    int findKthLargest(vector<int>& nums, int k) {
        k = nums.size() - k;
        int left = 0, right = nums.size() - 1;
        while (left < right) {
            int pivot = partition(nums, left, right);
            if (pivot < k)
                left = pivot + 1;
            else if (pivot > k)
                right = pivot - 1;
            else
                break;
        }
        return nums[k];
    }

private:
    int partition(vector<int>& nums, int left, int right) {
        int pivot = nums[right], fill = left;
        for (int i = left; i < right; i++) {
            if (nums[i] <= pivot) {
                swap(nums[fill++], nums[i]);
            }
        }
        swap(nums[right], nums[fill]);
        return fill;
    }
};
