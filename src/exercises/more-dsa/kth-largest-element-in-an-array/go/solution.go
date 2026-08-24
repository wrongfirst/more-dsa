func findKthLargest(nums []int, k int) int {
    k = len(nums) - k
    left, right := 0, len(nums)-1
    
    for left < right {
        pivot := partition(nums, left, right)
        
        if pivot < k {
            left = pivot + 1
        } else if pivot > k {
            right = pivot - 1
        } else {
            break
        }
    }
    
    return nums[k]
}

func partition(nums []int, left int, right int) int {
    pivot, fill := nums[right], left
    
    for i := left; i < right; i++ {
        if nums[i] <= pivot {
            nums[fill], nums[i] = nums[i], nums[fill]
            fill++
        }
    }
    
    nums[fill], nums[right] = nums[right], nums[fill]
    
    return fill
}
