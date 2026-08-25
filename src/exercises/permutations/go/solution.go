func permute(nums []int) [][]int {
    if len(nums) == 0 {
        return [][]int{{}}
    }

    perms := permute(nums[1:])
    var res [][]int
    for _, p := range perms {
        for i := 0; i <= len(p); i++ {
            pCopy := append([]int{}, p...)
            pCopy = append(pCopy[:i], append([]int{nums[0]}, pCopy[i:]...)...)
            res = append(res, pCopy)
        }
    }
    return res
}
