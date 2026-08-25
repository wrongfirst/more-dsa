func checkValidString(s string) bool {
    leftMin, leftMax := 0, 0

    for _, c := range s {
        if c == '(' {
            leftMin, leftMax = leftMin+1, leftMax+1
        } else if c == ')' {
            leftMin, leftMax = leftMin-1, leftMax-1
        } else {
            leftMin, leftMax = leftMin-1, leftMax+1
        }

        if leftMax < 0 {
            return false
        }
        if leftMin < 0 {
            leftMin = 0
        }
    }
    return leftMin == 0
}
