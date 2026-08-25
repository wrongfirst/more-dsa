func multiply(num1 string, num2 string) string {
	if num1 == "0" || num2 == "0" {
		return "0"
	}

	res := make([]int, len(num1)+len(num2))
	num1Rev := reverseString(num1)
	num2Rev := reverseString(num2)

	for i1 := 0; i1 < len(num1Rev); i1++ {
		for i2 := 0; i2 < len(num2Rev); i2++ {
			digit1 := int(num1Rev[i1] - '0')
			digit2 := int(num2Rev[i2] - '0')
			digit := digit1 * digit2
			res[i1+i2] += digit
			res[i1+i2+1] += res[i1+i2] / 10
			res[i1+i2] = res[i1+i2] % 10
		}
	}
	for l, r := 0, len(res)-1; l < r; l, r = l+1, r-1 {
		res[l], res[r] = res[r], res[l]
	}

	beg := 0
	for beg < len(res) && res[beg] == 0 {
		beg++
	}

	strRes := ""
	for _, v := range res[beg:] {
		strRes += strconv.Itoa(v)
	}
	return strRes
}

func reverseString(s string) string {
	b := []rune(s)
	for l, r := 0, len(b)-1; l < r; l, r = l+1, r-1 {
		b[l], b[r] = b[r], b[l]
	}
	return string(b)
}
