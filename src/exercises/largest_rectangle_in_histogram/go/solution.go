func largestRectangleArea(heights []int) int {
	maxArea := 0
	stack := [][]int{}  // pair: (index, height)

	for i, h := range heights {
		start := i
		for len(stack) > 0 && stack[len(stack)-1][1] > h {
			index := stack[len(stack)-1][0]
			height := stack[len(stack)-1][1]
			stack = stack[:len(stack)-1]
			area := height * (i - index)
			if area > maxArea {
				maxArea = area
			}
			start = index
		}
		stack = append(stack, []int{start, h})
	}

	for _, pair := range stack {
		i, h := pair[0], pair[1]
		area := h * (len(heights) - i)
		if area > maxArea {
			maxArea = area
		}
	}

	return maxArea
}
