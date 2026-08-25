type TimeMap struct {
	keyStore map[string][][]interface{}  // key : list of [value, timestamp] pairs
}

func Constructor() TimeMap {
	return TimeMap{
		keyStore: make(map[string][][]interface{}),
	}
}

func (this *TimeMap) Set(key string, value string, timestamp int) {
	if _, exists := this.keyStore[key]; !exists {
		this.keyStore[key] = [][]interface{}{}
	}
	this.keyStore[key] = append(this.keyStore[key], []interface{}{value, timestamp})
}

func (this *TimeMap) Get(key string, timestamp int) string {
	res := ""
	values, exists := this.keyStore[key]
	if !exists {
		values = [][]interface{}{}
	}

	l, r := 0, len(values)-1
	for l <= r {
		m := (l + r) / 2
		if values[m][1].(int) <= timestamp {
			res = values[m][0].(string)
			l = m + 1
		} else {
			r = m - 1
		}
	}

	return res
}
