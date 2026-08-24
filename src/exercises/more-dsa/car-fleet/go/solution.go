func carFleet(target int, position []int, speed []int) int {
	pair := make([][]int, len(position))
	for i := range position {
		pair[i] = []int{position[i], speed[i]}
	}

	sort.Slice(pair, func(i, j int) bool {
		return pair[i][0] > pair[j][0]
	})

	stack := []float64{}
	for _, ps := range pair {  // Reverse Sorted Order
		p, s := ps[0], ps[1]
		time := float64(target-p) / float64(s)
		stack = append(stack, time)
		if len(stack) >= 2 && stack[len(stack)-1] <= stack[len(stack)-2] {
			stack = stack[:len(stack)-1]
		}
	}

	return len(stack)
}
