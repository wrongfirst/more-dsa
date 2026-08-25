func evalRPN(tokens []string) int {
	stack := []int{}
	for _, c := range tokens {
		if c == "+" {
			a := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			b := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			stack = append(stack, a+b)
		} else if c == "-" {
			a := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			b := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			stack = append(stack, b-a)
		} else if c == "*" {
			a := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			b := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			stack = append(stack, a*b)
		} else if c == "/" {
			a := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			b := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			stack = append(stack, b/a)
		} else {
			num, _ := strconv.Atoi(c)
			stack = append(stack, num)
		}
	}
	return stack[0]
}
