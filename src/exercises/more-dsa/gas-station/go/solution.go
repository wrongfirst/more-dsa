func canCompleteCircuit(gas []int, cost []int) int {
    gasSum := 0
    costSum := 0
    for i := 0; i < len(gas); i++ {
        gasSum += gas[i]
        costSum += cost[i]
    }
    if gasSum < costSum {
        return -1
    }

    total := 0
    res := 0
    for i := 0; i < len(gas); i++ {
        total += (gas[i] - cost[i])
        if total < 0 {
            total = 0
            res = i + 1
        }
    }
    return res
}
