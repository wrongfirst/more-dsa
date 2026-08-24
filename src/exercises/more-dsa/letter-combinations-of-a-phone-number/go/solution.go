func letterCombinations(digits string) []string {
	res := []string{}
	digitToChar := map[string]string{
		"2": "abc",
		"3": "def",
		"4": "ghi",
		"5": "jkl",
		"6": "mno",
		"7": "qprs",
		"8": "tuv",
		"9": "wxyz",
	}

	var backtrack func(int, string)
	backtrack = func(i int, curStr string) {
		if len(curStr) == len(digits) {
			res = append(res, curStr)
			return
		}
		for _, c := range digitToChar[string(digits[i])] {
			backtrack(i+1, curStr+string(c))
		}
	}

	if len(digits) > 0 {
		backtrack(0, "")
	}
	return res
}
