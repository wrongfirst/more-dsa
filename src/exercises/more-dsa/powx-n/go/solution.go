func myPow(x float64, n int) float64 {
	var helper func(float64, int) float64
	helper = func(x float64, n int) float64 {
		if x == 0 {
			return 0
		}
		if n == 0 {
			return 1
		}

		res := helper(x*x, n/2)
		if n%2 != 0 {
			return x * res
		}
		return res
	}

	res := helper(x, abs(n))
	if n >= 0 {
		return res
	}
	return 1 / res
}

func abs(n int) int {
	if n < 0 {
		return -n
	}
	return n
}
