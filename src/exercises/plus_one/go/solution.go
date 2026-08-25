func plusOne(digits []int) []int {
	one := 1
	i := 0

	for l, r := 0, len(digits)-1; l < r; l, r = l+1, r-1 {
		digits[l], digits[r] = digits[r], digits[l]
	}

	for one != 0 {
		if i < len(digits) {
			if digits[i] == 9 {
				digits[i] = 0
			} else {
				digits[i] += 1
				one = 0
			}
		} else {
			digits = append(digits, one)
			one = 0
		}
		i++
	}

	for l, r := 0, len(digits)-1; l < r; l, r = l+1, r-1 {
		digits[l], digits[r] = digits[r], digits[l]
	}

	return digits
}
