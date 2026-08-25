func findMedianSortedArrays(nums1 []int, nums2 []int) float64 {
    A, B := nums1, nums2
    total := len(nums1) + len(nums2)
    half := total / 2
    
    if len(B) < len(A) {
        A, B = B, A
    }
    
    l, r := 0, len(A)
    
    for l <= r {
        i := (l + r) / 2
        j := half - i
        
        Aleft := math.Inf(-1)
        if i > 0 {
            Aleft = float64(A[i-1])
        }
        
        Aright := math.Inf(1)
        if i < len(A) {
            Aright = float64(A[i])
        }
        
        Bleft := math.Inf(-1)
        if j > 0 {
            Bleft = float64(B[j-1])
        }
        
        Bright := math.Inf(1)
        if j < len(B) {
            Bright = float64(B[j])
        }
        
        if Aleft <= Bright && Bleft <= Aright {
            if total%2 == 1 {
                return math.Min(Aright, Bright)
            }
            return (math.Max(Aleft, Bleft) + math.Min(Aright, Bright)) / 2
        } else if Aleft > Bright {
            r = i - 1
        } else {
            l = i + 1
        }
    }
    
    return 0
}
