func maxCoins(nums []int) int {
    cache := make(map[[2]int]int)
    nums = append([]int{1}, nums...)
    nums = append(nums, 1)

    for offset := 2; offset < len(nums); offset++ {
        for left := 0; left < len(nums)-offset; left++ {
            right := left + offset
            for pivot := left + 1; pivot < right; pivot++ {
                coins := nums[left] * nums[pivot] * nums[right]
                leftVal := 0
                if val, exists := cache[[2]int{left, pivot}]; exists {
                    leftVal = val
                }
                rightVal := 0
                if val, exists := cache[[2]int{pivot, right}]; exists {
                    rightVal = val
                }
                coins += leftVal + rightVal

                currentMax := 0
                if val, exists := cache[[2]int{left, right}]; exists {
                    currentMax = val
                }
                cache[[2]int{left, right}] = max(coins, currentMax)
            }
        }
    }

    result := 0
    if val, exists := cache[[2]int{0, len(nums) - 1}]; exists {
        result = val
    }
    return result
}
