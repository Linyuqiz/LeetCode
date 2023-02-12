package main

func main() {
	println("346、moving_average_from_data_stream")
}

// leetcode submit region begin(Prohibit modification and deletion)
type MovingAverage struct {
	L []int
	C int
}

func Constructor(size int) MovingAverage {
	return MovingAverage{
		L: make([]int, 0),
		C: size,
	}
}

func (this *MovingAverage) Next(val int) float64 {
	r := 0
	if len(this.L) == this.C {
		this.L = this.L[1:]
	}
	this.L = append(this.L, val)
	for _, v := range this.L {
		r = r + v
	}
	return float64(r) / float64(len(this.L))
}

/**
 * Your MovingAverage object will be instantiated and called as such:
 * obj := Constructor(size);
 * param_1 := obj.Next(val);
 */
//leetcode submit region end(Prohibit modification and deletion)
