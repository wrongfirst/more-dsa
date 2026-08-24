func partitionLabels(s string) []int {
    count := make(map[rune]int)
    res := []int{}
    i, length := 0, len(s)

    for j := 0; j < length; j++ {
        c := rune(s[j])
        count[c] = j
    }

    curLen := 0
    goal := 0
    for i < length {
        c := rune(s[i])
        goal = max(goal, count[c])
        curLen++

        if goal == i {
            res = append(res, curLen)
            curLen = 0
        }
        i++
    }
    return res
}
