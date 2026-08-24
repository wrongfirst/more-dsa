type CountSquares struct {
	ptsCount map[[2]int]int
	pts      [][2]int
}

func Constructor() CountSquares {
	return CountSquares{
		ptsCount: make(map[[2]int]int),
		pts:      make([][2]int, 0),
	}
}

func (this *CountSquares) Add(point []int) {
	pt := [2]int{point[0], point[1]}
	this.ptsCount[pt]++
	this.pts = append(this.pts, pt)
}

func (this *CountSquares) Count(point []int) int {
	res := 0
	px, py := point[0], point[1]

	for _, p := range this.pts {
		x, y := p[0], p[1]
		if abs(py-y) != abs(px-x) || x == px || y == py {
			continue
		}
		res += this.ptsCount[[2]int{x, py}] * this.ptsCount[[2]int{px, y}]
	}
	return res
}

func abs(n int) int {
	if n < 0 {
		return -n
	}
	return n
}
