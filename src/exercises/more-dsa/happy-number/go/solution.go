func isHappy(n int) bool {
	slow, fast := n, sumSquareDigits(n)

	for slow != fast {
		fast = sumSquareDigits(fast)
		fast = sumSquareDigits(fast)
		slow = sumSquareDigits(slow)
	}

	if fast == 1 {
		return true
	}
	return false
}

func sumSquareDigits(n int) int {
	output := 0
	for n != 0 {
		digit := n % 10
		output += digit * digit
		n = n / 10
	}
	return output
}
